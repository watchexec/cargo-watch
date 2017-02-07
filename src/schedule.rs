use cargo;
use config;
#[cfg(not(windows))]
use libc;
use notify::DebouncedEvent;
use std::io;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use wait_timeout::ChildExt;

/// Information about the currently executed job
#[derive(PartialEq, Debug, Clone, Copy)]
enum JobInfo {
    /// No job is executed
    Idle,
    /// A compile-ish job is executed (like `cargo build` or `cargo test`)
    Rustc,
    /// The user's application is executed (with `cargo run`)
    User,
}

/// Waits for changes in the directory and handles them (runs in main thread)
pub fn handle(rx: Receiver<DebouncedEvent>, mut commands: Vec<String>) {
    // If no commands were specified we use the default commands
    if commands.is_empty() {
        commands.extend(config::DEFAULT_COMMANDS.iter().map(|&s| s.into()));
    }

    // We need to wrap it into an Arc to safely share it with other threads.
    // This would be possible with scoped threads, but Arc works more easily
    // in this situation.
    let commands = Arc::new(commands);

    // Both threads need to know what kind of job is being executed, if any
    let job_info = Arc::new(Mutex::new(JobInfo::Idle));

    // The sender that can send kill signals to the processing thread
    let mut kill: Option<Sender<()>> = None;
    let mut thread: Option<JoinHandle<()>> = None;

    // At the start, run the command. cf. issue #37
    info!("First run, so starting a command run");
    // Create channel to send kill signals
    let (tx2, rx2) = channel();
    kill = Some(tx2);

    let thread_commands = commands.clone();
    let thread_job_info = job_info.clone();

    thread = Some(
        thread::spawn(move || {
            execute_commands(&thread_commands, thread_job_info, rx2)
        })
    );

    // Handle events as long as the watcher still sends events
    // through the channel
    while let Ok(event) = rx.recv() {
        // Check if the event refers to a valid file
        let path = match event {
            DebouncedEvent::Create(ref p) => p,
            DebouncedEvent::Write(ref p) => p,
            DebouncedEvent::Remove(ref p) => p,
            DebouncedEvent::Rename(_, ref p) => p,
            _ => continue,
        };
        debug!("path changed: {}", path.display());
        let filename = match path.file_name() {
            Some(f) => f,
            None => continue,
        };

        // Check if this file should be ignored
        let filename = filename.to_string_lossy();
        if cargo::is_ignored_file(&filename) {
            info!("Ignoring change on '{}' ({})", filename, path.display());
            continue;
        }

        info!("Request to spawn a job");

        // Check if another job is already in execution
        let mut job = job_info.lock().unwrap();
        if *job == JobInfo::Rustc {
            info!("Another command is currently in execution: request denied");
            continue;
        } else if *job == JobInfo::User {
            // Send kill signal. We can unwrap here, because `*job` is `Idle`
            // until the channel is created. The result can be ignored: an
            // error means that the other end has hung up. Since we hold the
            // lock of `job`, this means that the other thread has panicked.
            // The other end is deallocated when `job` is reset to `Idle`. The
            // panicking case is handled below.
            let _ = kill.unwrap().send(());

            // After the kill signal was send, we have to wait for the thread
            // to receive it and terminate itself. About unwrap: see above.
            // We have to reset the job info ourselves, because the other
            // thread is unable to, because we are holding the lock.
            let res = thread.unwrap().join();
            *job = JobInfo::Idle;
            if res.is_err() {
                info!("child thread panicked...")
            }
        }

        // No other job is in execution: start new one.
        *job = JobInfo::Rustc;

        // Create channel to send kill signals
        let (tx, rx) = channel();
        kill = Some(tx);

        let thread_commands = commands.clone();
        let thread_job_info = job_info.clone();

        thread = Some(
            thread::spawn(move || {
                execute_commands(&thread_commands, thread_job_info, rx)
            })
        );
    }
}

/// Executes given commands in order (runs on extra thread)
fn execute_commands(
    commands: &[String],
    job_info: Arc<Mutex<JobInfo>>,
    kill: Receiver<()>,
) {
    // helper to update the "global" job information
    let update_info = |new_info: JobInfo| {
        // We can unwrap here: the only way `lock` returns an `Err` value is
        // when the main thread has panicked. In this case this thread should
        // panic, too.
        let mut guard = job_info.lock().unwrap();
        *guard = new_info;
    };

    debug!("Starting a command run!");

    // Says if we received a kill signal
    let mut abort = false;

    // Iterate through all given commands and execute them in order
    for command in commands {
        if abort {
            debug!("Aborting command run");
            break;
        }
        let args: Vec<&str> = command.split_whitespace().collect();

        println!("");
        println!("$ cargo {}", command);

        // Update global information about the running job
        if args.get(0) == Some(&"run") {
            update_info(JobInfo::User);
        } else {
            update_info(JobInfo::Rustc);
        }

        let mut status = || -> Result<_, io::Error> {
            // Start the process
            let mut child = try!(Command::new("cargo").args(&args).spawn());

            loop {
                // Wait some time for it to finish
                let res = child.wait_timeout_ms(config::PROCESS_WAIT_TIMEOUT);

                // Check if kill signal was received
                if let Ok(_) = kill.try_recv() {
                    // We got the order to kill the child and terminate
                    debug!("Killing spawned process");
                    kill_child(&mut child);
                    abort = true;
                }

                // If the wait finished with an error or returned because the
                // process ended or if we need to abort, we return the wait
                // result directly.
                if res.is_err() || res.as_ref().unwrap().is_some() || abort {
                    return res;
                }
            }
        };

        match status() {
            Ok(None) => info!("Process did not end normally, was killed"),
            Ok(Some(status)) => println!("-> {}", status),
            Err(e) => {
                println!(
                    "Failed to execute 'cargo {}': {}",
                    command,
                    e
                );
            },
        }
    }
    // Reset job info, if it wasn't caused by a kill signal. If it was, the
    // main thread still holds the lock and is trying to join this thread,
    // which results in a deadlock.
    if !abort {
        update_info(JobInfo::Idle);
    }

    debug!("Command run done");
}

#[cfg(windows)]
fn kill_child(child: &mut Child) -> () {
    let _ = child.kill();
}

#[cfg(not(windows))]
fn kill_child(child: &mut Child) -> () {
    let _ = unsafe { libc::kill(child.id() as i32, libc::SIGTERM) };
}

