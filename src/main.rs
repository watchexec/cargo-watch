//! Watch files in a Cargo project and compile it when they change

#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

use clap::ArgMatches;
use std::env;
use std::process::{Command, exit};

mod args;
mod cargo;

fn get_command(matches: &ArgMatches) -> String {
    let cargo_dir = cargo::root().unwrap_or_else(|| {
        error!("Not a Cargo project, aborting.");
        exit(64);
    });

    let mut commands: Vec<String> = vec![];

    // Cargo commands are in front of the rest
    if matches.is_present("cmd:cargo") {
        for cargo in values_t!(matches, "cmd:cargo", String).unwrap_or_else(|e| e.exit()) {
            let mut cmd: String = "cargo ".into();
            cmd.push_str(&cargo);
            commands.push(cmd);
        }
    }

    // Shell/raw commands go last
    if matches.is_present("cmd:shell") {
        for shell in values_t!(matches, "cmd:shell", String).unwrap_or_else(|e| e.exit()) {
            commands.push(shell);
        }
    }

    // Default to `cargo check`
    if commands.is_empty() {
        commands.push("cargo check".into());
    }

    info!("Commands: {:?}", commands);

    if !matches.is_present("quiet") {
        let start = {
            format!("echo [Running '{}']", commands.join(" && "))
        };

        commands.insert(0, start);
        commands.push("echo [Finished running]".into());
    }

    commands.insert(0, format!("cd {}", cargo_dir.display()));
    commands.join(" && ")
}

fn get_filter(matches: &ArgMatches) -> Vec<String> {
    let mut opts: Vec<String> = vec![];

    if matches.is_present("ignore-nothing") {
        return vec!["--no-vcs-ignore".into()];
    }

    if matches.is_present("no-gitignore") {
        opts.push("--no-vcs-ignore".into());

        opts.push("--ignore".into());
        opts.push("target".into());

        opts.push("--ignore".into());
        opts.push(".git".into());
    }

    if matches.is_present("ignore") {
        for ignore in values_t!(matches, "ignore", String).unwrap_or_else(|e| e.exit()) {
            opts.push("--ignore".into());
            opts.push(ignore);
        }
    }

    info!("Filters: {:?}", opts);
    opts
}

fn get_settings(matches: &ArgMatches) -> Vec<String> {
    let mut opts: Vec<String> = vec![
        "--restart".into()
    ];

    if matches.is_present("clear") {
        opts.push("--clear".into());
    }

    if matches.is_present("postpone") {
        opts.push("--postpone".into());
    }

    if matches.is_present("poll") {
        let delay = value_t!(matches, "delay", u64).unwrap_or_else(|e| e.exit());
        info!("Delay: {} seconds", delay);
        opts.push("--force-poll".into());
        opts.push(format!("{}", delay));
    }

    if let Ok(_) = env::var("RUST_LOG") {
        opts.push("--debug".into());
    }

    info!("Settings: {:?}", opts);
    opts
}

fn get_watches(matches: &ArgMatches) -> Vec<String> {
    let mut opts: Vec<String> = vec![];
    if matches.is_present("watch") {
        for watch in values_t!(matches, "watch", String).unwrap_or_else(|e| e.exit()) {
            opts.push("--watch".into());
            opts.push(watch);
        }
    }

    info!("Watches: {:?}", opts);
    opts
}

fn get_options(matches: &ArgMatches) -> Vec<String> {
    let mut opts: Vec<String> = vec![];
    opts.append(&mut get_filter(matches));
    opts.append(&mut get_settings(matches));
    opts.append(&mut get_watches(matches));
    opts.push(get_command(&matches));
    opts
}

fn main() {
    let matches = args::parse();
    env_logger::init().unwrap();

    let opts = get_options(&matches);
    let status = Command::new("watchexec")
                        .args(&opts)
                        .status()
                        .expect("Failed to execute watchexec! Make sure it's installed and in your PATH.");
    
    if !status.success() {
        error!("Oh no! Watchexec exited with: {}", status);
    }
}
