//! Watch files in a Cargo project and compile it when they change

#[macro_use]
extern crate clap;
#[macro_use]
extern crate duct;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate notify;
extern crate regex;
extern crate rustc_serialize;
extern crate wait_timeout;

use schedule::{Command, Setting, Settings};
use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::channel;
use std::time::Duration;
use watcher::DualWatcher;

mod args;
mod cargo;
mod config;
mod schedule;
mod watcher;

fn main() {
    let matches = args::parse();
    env_logger::init().unwrap();

    // Compute settings for the scheduler
    let mut settings: Settings = vec![];

    if matches.is_present("postpone") {
        settings.push(Setting::Postpone);
    }

    if matches.is_present("quiet") {
        settings.push(Setting::Quiet);
    }

    if matches.is_present("watch") {
        settings.push(Setting::NoIgnores);
    }


    // Build up command set
    let mut commands: Vec<Command> = vec![];

    // Flagged clear always comes first
    if matches.is_present("clear") {
        commands.push(Command::Clear);
    }

    // Cargo commands are in front of the rest
    for cargo in match matches.is_present("cmd:cargo") {
        false => vec![],
        true => values_t!(matches, "cmd:cargo", String)
            .unwrap_or_else(|e| e.exit())
    }.into_iter() {
        commands.push(if cargo == "clear" {
            Command::Clear
        } else {
            Command::Cargo(cargo)
        });
    }

    // Shell/raw commands go last
    for shell in match matches.is_present("cmd:shell") {
        false => vec![],
        true => values_t!(matches, "cmd:shell", String)
            .unwrap_or_else(|e| e.exit())
    }.into_iter() {
        commands.push(if shell == "clear" {
            Command::Clear
        } else {
            Command::Shell(shell)
        });
    }

    // Check if we are (somewhere) in a cargo project directory
    let cargo_dir = match cargo::root() {
        Some(path) => path,
        None => {
            error!("Not a Cargo project, aborting.");
            exit(64);
        },
    };

    // Options relevant to creating the Watcher
    let delay = value_t!(matches, "delay", u8).unwrap_or_else(|e| e.exit());
    let poll = matches.is_present("poll");

    // Creates Watcher instance and a channel to communicate with it
    let (tx, rx) = channel();
    let d = Duration::from_secs(delay as u64);
    let mut watcher = if poll {
        DualWatcher::fallback_only(tx, d)
    } else {
        DualWatcher::new(tx, d)
    };

    // Convert string watches to paths… or defaults
    let watches = match matches.is_present("watch") {
        false => config::default_watches(),
        true => values_t!(matches, "watch", String)
            .and_then(|s| Ok(s
                .into_iter()
                .map(|s| s.into())
                .collect::<Vec<PathBuf>>()
            ))
            .unwrap_or_else(|e| e.exit())
    };

    // Configure Watcher: we want to monitor these
    for subdir in watches {
        // We ignore any errors (e.g. if the directory doesn't exist)
        let _ = watcher.watch(&cargo_dir.join(subdir));
    }

    debug!("{:?}", commands);

    // Handle incoming events from the watcher
    schedule::handle(rx, commands, settings);
}
