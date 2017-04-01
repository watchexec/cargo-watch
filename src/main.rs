//! Watch files in a Cargo project and compile it when they change

#[macro_use]
extern crate clap;
extern crate duct;
extern crate env_logger;
extern crate ignore;
#[macro_use]
extern crate log;
extern crate notify;
extern crate walkdir;

use clap::ArgMatches;
use filter::Filter;
use schedule::{Command, Setting, Settings};
use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::channel;
use std::time::Duration;
use watcher::{DualWatcher, Sender};

mod args;
mod cargo;
mod filter;
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

    info!("Commands: {:?}", commands);
    commands
}

fn get_filter(matches: &ArgMatches) -> Filter {
    debug!("Getting current working dir");
    let cwd = env::current_dir().unwrap_or_else(|e| {
        error!("Cannot get current working dir, aborting.");
        error!("{}", e);
        exit(1);
    });

    let (patterns, walktree): (Vec<String>, bool) =
    if matches.is_present("ignore-nothing") {
        (vec![], false)
    } else {
        debug!("Enabling filter from gitignores");
        let gitignores = !matches.is_present("no-gitignore");

        let mut patterns: Vec<String> = vec![
            "/.git/**".into(),
            "/target/**".into()
        ];

        if matches.is_present("ignore") {
            for pattern in values_t!(matches, "ignore", String).unwrap_or_else(|e| e.exit()) {
                patterns.push(pattern);
            }
        }

        (patterns, gitignores)
    };

    info!("Filters: {:?}", patterns);
    Filter::create(cwd, patterns, walktree)
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

    info!("Settings: {:?}", settings);
    settings
}

fn get_watches(matches: &ArgMatches) -> Vec<PathBuf> {
    let cargo_dir = cargo::root().unwrap_or_else(|| {
        error!("Not a Cargo project, aborting.");
        exit(64);
    });

    let watches: Vec<String> = if matches.is_present("watch") {
        values_t!(matches, "watch", String).unwrap_or_else(|e| e.exit())
    } else {
        vec!["".into()]
    };

    let watches = watches.into_iter().map(|p|
        cargo_dir.clone().join(PathBuf::from(if p == "." {
            "".into() // Normalise root path
        } else {
            p
        }))
    ).collect::<Vec<PathBuf>>();

    info!("Watches: {:?}", watches);
    watches
}

fn make_watcher(matches: &ArgMatches, tx: Sender) -> DualWatcher {
    // Options relevant to creating the Watcher
    let poll = matches.is_present("poll");
    let delay = value_t!(matches, "delay", u64)
        .and_then(|d| {
            info!("Delay: {} seconds", d);
            Ok(Duration::from_secs(d))
        })
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
    let filter = get_filter(&matches);
    let settings = get_settings(&matches);

    // Handle incoming events from the watcher
    schedule::handle(&rx, commands, &filter, &settings);
}
