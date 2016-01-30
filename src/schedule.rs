use cargo;
use config;
use notify;
use std::io;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
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
pub fn handle(rx: Receiver<notify::Event>, mut commands: Vec<String>) {
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

    // Handle events as long as the watcher still sends events
    // through the channel
    while let Ok(event) = rx.recv() {
        // Check if the event refers to a valid file
        let path = match event.path {
            Some(ref p) => p,
            None => continue,
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
        if *job != JobInfo::Idle {
            info!("Another command is currently in execution: request denied");
            continue;
        }

        // No other job is in execution: start new one.
        *job = JobInfo::Rustc;

        let thread_commands = commands.clone();
        let thread_job_info = job_info.clone();

        let _ = thread::spawn(move || {
            execute_commands(&thread_commands, thread_job_info)
        });
    }
}

/// Executes given commands in order (runs on extra thread)
fn execute_commands(commands: &[String], job_info: Arc<Mutex<JobInfo>>) {
    // helper to update the "global" job information
    let update_info = |new_info: JobInfo| {
        // We can unwrap here: the only way `lock` returns an `Err` value is
        // when the main thread has panicked. In this case this thread should
        // panic, too.
        let mut guard = job_info.lock().unwrap();
        *guard = new_info;
    };

    debug!("Starting a command run!");

    // Iterate through all given commands and execute them in order
    for command in commands {
        let args: Vec<&str> = command.split_whitespace().collect();

        println!("");
        println!("$ cargo {}", command);

        // Update global information about the running job
        if args.get(0) == Some(&"run") {
            update_info(JobInfo::User);
        } else {
            update_info(JobInfo::Rustc);
        }

        let status = || -> Result<_, io::Error> {
            // Start the process
            let mut child = try!(Command::new("cargo").args(&args).spawn());

            loop {
                // Wait some time for it to finish
                let res = child.wait_timeout_ms(config::PROCESS_WAIT_TIMEOUT);

                // TODO: check if kill signal was received

                // If the wait finished with an error or returned successfully,
                // we return from this function.
                if let Some(s) = try!(res) {
                    return Ok(Some(s));
                }
            }
        };

        match status() {
            Ok(None) => {},
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
    // Reset job info
    update_info(JobInfo::Idle);

    debug!("Command run done");
}
