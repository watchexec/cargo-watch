use std::sync::Arc;
use std::sync::atomic::AtomicIsize;
use std::thread;
use super::{Config, cargo, ignore, notify, timelock};

fn compile(t: Arc<AtomicIsize>, c: Arc<Config>) {
    let Config { ref args } = *c;
    debug!("Starting a compile");

    // TODO: This still has allocation overhead -> remove it
    let commands = if args.len() > 0 {
        args.iter().map(|s| &s[..]).collect()
    } else {
        vec!["build", "test"]
    };

    for command in commands {
        cargo::run(command);
    }

    timelock::update(&t);
    debug!("Compile done");
}

fn spawn_compile(t: &Arc<AtomicIsize>, c: Arc<Config>) {
    info!("Request to spawn a compile");

    // Don't run compiles within less than 2s of each other
    // TODO: put this constant somewhere else
    let justnow = timelock::current() - 2;
    let prev = timelock::get(t);
    if prev > justnow {
        info!("Request denied (last request was less than 2 seconds ago)");
    } else {
        timelock::update(t);
        let t2 = t.clone();
        let _ = thread::spawn(move || {
            compile(t2, c);
        });
    }
}

pub fn handle_event(t: &Arc<AtomicIsize>, e: notify::Event, c: Arc<Config>) {
    // Check if the event refers to a valid file
    if let Some(path) = e.path {
        debug!("path: {}", path.display());

        if let Some(filename) = path.file_name() {
            // Check if this file should be ignored
            let filename = filename.to_string_lossy();
            if ignore::filename(&filename) {
                info!("Ignoring change on '{}' ({})", filename, path.display());
            } else {
                spawn_compile(t, c);
            }
        }
    }
}
