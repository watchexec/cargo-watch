//! Watch files in a Cargo project and compile it when they change

#[macro_use]
extern crate clap;
extern crate watchexec;

use clap::{ArgMatches, Error, ErrorKind};
use std::env::set_current_dir;
use std::path::MAIN_SEPARATOR;
use watchexec::cli::Args;

mod args;
mod cargo;

fn change_dir() {
    cargo::root()
        .and_then(|p| set_current_dir(p).ok())
        .unwrap_or_else(|| {
            Error::with_description("Not a Cargo project, aborting.", ErrorKind::Io).exit();
        });
}

fn get_command(debug: bool, matches: &ArgMatches) -> String {
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

    commands.join(" && ")
}

fn get_ignores(debug: bool, matches: &ArgMatches) -> (bool, Vec<String>) {
    let mut opts: Vec<String> = vec![];

    if matches.is_present("ignore-nothing") {
        if debug {
            println!(">>> Ignoring nothing");
        }

        return (true, vec![]);
    }

    let novcs = matches.is_present("no-gitignore");
    if debug {
        println!(">>> Load Git/VCS ignores: {:?}", !novcs);
    }

    opts.push(format!("*{}.DS_Store", MAIN_SEPARATOR));
    opts.push("*.swp".into());

    opts.push(format!("*{s}.hg{s}**", s = MAIN_SEPARATOR));
    opts.push(format!("*{s}.git{s}**", s = MAIN_SEPARATOR));
    opts.push(format!("*{s}.svn{s}**", s = MAIN_SEPARATOR));

    opts.push(format!("*{s}target{s}**", s = MAIN_SEPARATOR));

    if debug {
        println!(">>> Default ignores: {:?}", opts);
    }

    if matches.is_present("ignore") {
        for ignore in values_t!(matches, "ignore", String).unwrap_or_else(|e| e.exit()) {
            #[cfg(windows)] let ignore = ignore.replace("/", &MAIN_SEPARATOR.to_string());
            opts.push(ignore);
        }
    }

    if debug {
        println!(">>> All ignores: {:?}", opts);
    }

    (novcs, opts)
}

fn get_debounce(debug: bool, matches: &ArgMatches) -> u64 {
    if matches.is_present("delay") {
        let debounce = value_t!(matches, "delay", f64).unwrap_or_else(|e| e.exit());
        if debug {
            println!(">>> File updates debounce: {} seconds", debounce);
        }
        (debounce * 1000f64) as u64
    } else {
        500
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
    let debounce = get_debounce(debug, &matches);

    let args = Args {
        filters: vec![],
        no_shell: false,
        once: false,
        signal: None,
        restart: !matches.is_present("no-restart"),

        poll: matches.is_present("poll"),
        poll_interval: debounce as u32,
        debounce: debounce,

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

    change_dir();

    let debug = matches.is_present("debug");
    let opts = get_options(debug, &matches);
    watchexec::run(opts)
}
