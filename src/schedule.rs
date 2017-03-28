use cargo;
use config;
use duct::{Expression, sh};
use notify::DebouncedEvent;
use std::io;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use wait_timeout::ChildExt;

#[derive(Debug)]
pub enum Command {
    Clear,
    Cargo(String),
    Shell(String),
    Duct(Expression),
}

impl Command {
    pub fn is_clear(&self) -> bool {
        match *self {
            Command::Clear => true,
            _ => false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Setting {
    NoIgnores,
    Postpone,
    Quiet,
}

pub type Settings = Vec<Setting>;

/// Waits for changes in the directory and handles them (runs in main thread)
pub fn handle(rx: Receiver<DebouncedEvent>, commands: Vec<Command>, settings: &Settings) {
    let commands = commands.into_iter().map(|c| match c {
        Command::Cargo(s) => {
            let mut cmd: String = "cargo-".into();
            cmd.push_str(&s);
            Command::Duct(sh(cmd))
        },
        Command::Shell(cmd) => Command::Duct(sh(cmd)),
        c => c
    });

    // We need to wrap it into an Arc to safely share it with other threads.
    // This would be possible with scoped threads, but Arc works more easily
    // in this situation.
    let commands = Arc::new(commands);

    // Both threads need to know if a job is being executed.
    let job_info = Arc::new(AtomicBool::new(false));

    // The sender that can send kill signals to the processing thread.
    let mut kill: Option<Sender<()>> = None;
    let mut thread: Option<JoinHandle<()>> = None;

    // Handle events as long as the watcher still sends events
    // through the channel
    while let Ok(event) = rx.recv() {
        // Check if the event refers to a valid file
        let path = match event {
            DebouncedEvent::Create(ref p)
            | DebouncedEvent::Write(ref p)
            | DebouncedEvent::Remove(ref p)
            | DebouncedEvent::Rename(_, ref p) => p,
            _ => continue,
        };
        debug!("path changed: {}", path.display());
        let filename = match path.file_name() {
            Some(f) => f,
            None => continue,
        }.to_string_lossy();

        // Check if this file should be ignored
        if !settings.contains(&Setting::NoIgnores) && cargo::is_ignored_file(&filename) {
            info!("Ignoring change on '{}' ({})", filename, path.display());
            continue;
        }

        info!("Request to spawn a job");

        /*
        // Check if another job is already in execution
        let mut job = job_info.lock().unwrap();
        if *job {
            // TODO: Kill the previous process.
        }

        // No other job is in execution: start new one.
        *job = true;

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
        */
    }

}

/*
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

        // If we need to clear, send instead the terminal escape.
        if args == vec!["clear"] {
            print!("\u{001b}c");
            continue;
        } else {
            println!("");
        }

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
*/
