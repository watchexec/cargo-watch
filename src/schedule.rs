#[allow(deprecated)]
use duct::{Expression, Handle, sh};
use filter::Filter;
use notify::DebouncedEvent;
use std::process::exit;
use std::sync::mpsc::Receiver;

#[derive(Debug)]
pub enum Command {
    Cargo(String),
    Shell(String)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Setting {
    Clear,
    Postpone,
    Quiet,
}

pub type Settings = Vec<Setting>;

fn filtered(filter: &Filter, event: &DebouncedEvent) -> bool {
    let path = match *event {
        DebouncedEvent::Create(ref p)
        | DebouncedEvent::Write(ref p)
        | DebouncedEvent::Remove(ref p)
        | DebouncedEvent::Rename(_, ref p) => p,
        _ => return true,
    };

    debug!("path changed: {}", path.display());

    if filter.matched(&path) {
        info!("Ignoring change on {}", path.display());
        return true;
    }

    false
}

#[allow(deprecated)]
fn linearise(commands: Vec<Command>, quiet: bool) -> Expression {
    // Keep a record of command lines
    let mut lines: Vec<String> = vec![];

    // Convert all commands into Duct expressions.
    let mut exprs: Vec<Expression> = vec![];

    for command in commands {
        match command {
            Command::Shell(cmd) => {
                exprs.push(sh(&cmd));
                lines.push(cmd);
            },
            Command::Cargo(s) => {
                // TODO: Use duct::cmd() instead by resolving $(which cargo)
                let mut cmd: String = "cargo ".into();
                cmd.push_str(&s);
                exprs.push(sh(&cmd));
                lines.push(cmd);
            }
        }
    }

    let mut expr = exprs.remove(0);

    if !quiet {
        expr = sh(
            format!("echo [Running '{}']", lines.join(" && ").replace("'", "\""))
        ).then(expr);
    }

    for e in exprs {
        expr = expr.then(e);
    }

    if !quiet {
        expr = expr.unchecked().then(sh("echo [Finished running]"));
    }

    expr
}

fn start_job(expr: &Expression, quiet: bool) -> Option<Handle> {
    info!("Starting job: {:?}", expr);
    expr.start().or_else(|e| {
        if !quiet {
            error!("Couldn't start, waiting for file change");
            error!("{}", e);
        }

        Err(())
    }).ok()
}

pub fn handle(rx: &Receiver<DebouncedEvent>, commands: Vec<Command>, filter: &Filter, settings: &Settings) {
    // Convenience short bools for settings
    let clear = settings.contains(&Setting::Clear);
    let postpone = settings.contains(&Setting::Postpone);
    let quiet = settings.contains(&Setting::Quiet);

    // Get a single Duct expression from all the commands
    let expr = linearise(commands, quiet);

    // Keep track of the current running job.
    let mut job: Option<Handle> = None;

    if !quiet {
        println!("[Watching for changes... Ctrl-C to stop]");
    }

    if !postpone {
        job = start_job(&expr, quiet);
    }

    info!("Starting main loop");
    while let Ok(event) = rx.recv() {
        if filtered(filter, &event) {
            continue;
        }

        if !quiet {
            if let Some(ref handle) = job {
                debug!("Found a duct handle, checking if it's still running");
                let status = handle.try_wait().unwrap_or_else(|e| {
                    error!("Error trying to check status of job.");
                    error!("If this happens consistently, please file a bug.");
                    error!("https://github.com/passcod/cargo-watch/issues");
                    error!("{}", e);
                    None
                });

                if status.is_none() {
                    println!("[Killing running command]");
                    handle.kill().unwrap_or_else(|e| {
                        error!("Couldn't kill, abort.");
                        error!("{}", e);
                        exit(1);
                    });

                    info!("Waiting on the kill");
                    handle.wait().unwrap_or_else(|e| {
                        error!("Something went wrong after waiting on child.");
                        error!("Aborting to avoid zombification.");
                        error!("If this happens consistently, please file a bug.");
                        error!("https://github.com/passcod/cargo-watch/issues");
                        error!("{}", e);
                        exit(1);
                    });
                }
            }

            if clear {
                print!("\u{001b}c");
            } else {
                println!("");
            }
        }

        job = start_job(&expr, quiet);
    }
}
