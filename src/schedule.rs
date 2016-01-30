use cargo;
use config;
use notify;
use std::process::Command;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;

pub fn handle(rx: Receiver<notify::Event>, mut commands: Vec<String>) {
    // If no commands were specified we use the default commands
    if commands.is_empty() {
        commands.extend(config::DEFAULT_COMMANDS.iter().map(|&s| s.into()));
    }

    // We need to wrap it into an Arc to safely share it with other threads.
    // This would be possible with scoped threads, but Arc works more easily
    // in this situation.
    let commands = Arc::new(commands);

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
        let thread_commands = commands.clone();

        // TODO: check if another command is still running!
        let _ = thread::spawn(move || {
            debug!("Starting a compile");

            // Iterate through all given commands and execute them in order
            for command in &*thread_commands {
                let args: Vec<&str> = command.split_whitespace().collect();

                println!("");
                println!("$ cargo {}", command);

                // Start the process, wait for it and print the result
                let status = Command::new("cargo")
                                     .args(&args)
                                     .status();
                match status {
                    Ok(status) => println!("-> {}", status),
                    Err(e) => {
                        println!(
                            "Failed to execute 'cargo {}': {}",
                            command,
                            e
                        );
                    },
                };
            }

            debug!("Compile done");
        });
    }
}
