//! Watch files in a Cargo project and compile it when they change

#[macro_use]
extern crate clap;
extern crate watchexec;

use clap::{ArgMatches, Error, ErrorKind};
use std::path::MAIN_SEPARATOR;
use watchexec::cli::Args;

mod args;
mod cargo;

fn get_command(debug: bool, matches: &ArgMatches) -> String {
    let cargo_dir = cargo::root().unwrap_or_else(|| {
        Error::with_description(
            "Not a Cargo project, aborting.",
            ErrorKind::Io
        ).exit();
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

    if debug {
        println!(">>> Commands: {:?}", commands);
    }

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

fn get_ignores(debug: bool, matches: &ArgMatches) -> (bool, Vec<String>) {
    let mut opts: Vec<String> = vec![];

    if matches.is_present("ignore-nothing") {
        return (true, vec![]);
    }

    opts.push(format!("*{}.DS_Store", MAIN_SEPARATOR));
    opts.push("*.swp".into());
    opts.push(".git".into());
    opts.push("target".into());

    if matches.is_present("ignore") {
        for ignore in values_t!(matches, "ignore", String).unwrap_or_else(|e| e.exit()) {
            opts.push(ignore);
        }
    }

    let novcs = matches.is_present("no-gitignore");

    if debug {
        println!(">>> No VCS ignores: {:?}", novcs);
        println!(">>> Ignores: {:?}", opts);
    }

    (novcs, opts)
}

fn get_poll(debug: bool, matches: &ArgMatches) -> (bool, u32) {
    if matches.is_present("poll") {
        let delay = value_t!(matches, "delay", u32).unwrap_or_else(|e| e.exit());

        if debug {
            println!(">>> Poll with delay: {} seconds", delay);
        }

        (true, delay)
    } else {
        (false, 1000)
    }
}

fn get_watches(debug: bool, matches: &ArgMatches) -> Vec<String> {
    let mut opts: Vec<String> = vec![];
    if matches.is_present("watch") {
        for watch in values_t!(matches, "watch", String).unwrap_or_else(|e| e.exit()) {
            opts.push(watch);
        }
    }

    if debug {
        println!(">>> Watches: {:?}", opts);
    }

    opts
}

fn get_options(debug: bool, matches: &ArgMatches) -> Args {
    let (novcs, ignores) = get_ignores(debug, &matches);
    let (poll, delay) = get_poll(debug, &matches);

    let args = Args {
        filters: vec![],
        no_shell: false,
        once: false,
        signal: None,
        restart: true,

        poll: poll,
        poll_interval: delay,

        ignores: ignores,
        no_vcs_ignore: novcs,

        clear_screen: matches.is_present("clear"),
        debug: debug,
        run_initially: !matches.is_present("postpone"),

        cmd: get_command(debug, &matches),
        paths: get_watches(debug, &matches),
    };

    if debug {
        println!(">>> Watchexec arguments: {:?}", args);
    }

    args
}

fn main() {
    let matches = args::parse();
    let debug = matches.is_present("debug");

    let opts = get_options(debug, &matches);
    watchexec::run(opts)
}
