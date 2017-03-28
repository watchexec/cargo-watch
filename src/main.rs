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

use clap::ArgMatches;
use schedule::{Command, Setting, Settings};
use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::channel;
use std::time::Duration;
use watcher::{DualWatcher, Sender};

mod args;
mod cargo;
mod config;
mod schedule;
mod watcher;

fn get_commands(matches: &ArgMatches) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];

    // Cargo commands are in front of the rest
    if matches.is_present("cmd:cargo") {
        for cargo in values_t!(matches, "cmd:cargo", String).unwrap_or_else(|e| e.exit()) {
            commands.push(Command::Cargo(cargo));
        }
    }

    // Shell/raw commands go last
    if matches.is_present("cmd:shell") {
        for shell in values_t!(matches, "cmd:shell", String).unwrap_or_else(|e| e.exit()) {
            commands.push(Command::Shell(shell));
        }
    }

    // Default to `cargo check`
    if commands.is_empty() {
        commands.push(Command::Cargo("check".into()));
    }

    debug!("Commands: {:?}", commands);
    commands
}

fn get_settings(matches: &ArgMatches) -> Settings {
    let mut settings: Settings = vec![];

    if matches.is_present("clear") {
        settings.push(Setting::Clear);
    }

    if matches.is_present("postpone") {
        settings.push(Setting::Postpone);
    }

    if matches.is_present("quiet") {
        settings.push(Setting::Quiet);
    }

    if matches.is_present("watch") {
        settings.push(Setting::NoIgnores);
    }

    settings
}

fn get_watches(matches: &ArgMatches) -> Vec<PathBuf> {
    let cargo_dir = cargo::root().unwrap_or_else(|| {
        error!("Not a Cargo project, aborting.");
        exit(64);
    });

    if matches.is_present("watch") {
        values_t!(matches, "watch", String)
            .and_then(|s| Ok(s
                .into_iter()
                .map(|p| cargo_dir.clone().join(PathBuf::from(p)))
                .collect::<Vec<PathBuf>>()
            ))
            .unwrap_or_else(|e| e.exit())
    } else {
        config::default_watches()
    }
}

fn make_watcher(matches: &ArgMatches, tx: Sender) -> DualWatcher {
    // Options relevant to creating the Watcher
    let poll = matches.is_present("poll");
    let delay = value_t!(matches, "delay", u64)
        .and_then(|d| Ok(Duration::from_secs(d)))
        .unwrap_or_else(|e| e.exit());

    if poll {
        DualWatcher::fallback_only(tx, delay)
    } else {
        DualWatcher::new(tx, delay)
    }
}

fn main() {
    let matches = args::parse();
    env_logger::init().unwrap();

    let (tx, rx) = channel();
    let mut watcher = make_watcher(&matches, tx);

    let watches = get_watches(&matches);
    for dir in watches {
        // We ignore any errors (e.g. if the directory doesn't exist)
        let _ = watcher.watch(dir);
    }

    // Build up options from arguments
    let commands = get_commands(&matches);
    let settings = get_settings(&matches);

    // Handle incoming events from the watcher
    schedule::handle(rx, commands, &settings);
}
