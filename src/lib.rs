//! Watch files in a Cargo project and compile it when they change
#![forbid(unsafe_code, clippy::pedantic)]
#![allow(
    clippy::non_ascii_literal,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]

#[macro_use]
extern crate clap;
extern crate watchexec;

use clap::{ArgMatches, Error, ErrorKind};
use std::{env::set_current_dir, path::MAIN_SEPARATOR};
use watchexec::{Args, ArgsBuilder};

pub mod args;
pub mod cargo;
pub mod watch;

pub fn change_dir() {
    cargo::root()
        .and_then(|p| set_current_dir(p).ok())
        .unwrap_or_else(|| {
            Error::with_description("Not a Cargo project, aborting.", ErrorKind::Io).exit();
        });
}

pub fn set_commands(debug: bool, builder: &mut ArgsBuilder, matches: &ArgMatches) {
    let mut commands: Vec<String> = Vec::new();

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

    builder.cmd(commands);
}

pub fn set_ignores(debug: bool, builder: &mut ArgsBuilder, matches: &ArgMatches) {
    if matches.is_present("ignore-nothing") {
        if debug {
            println!(">>> Ignoring nothing");
        }

        builder.no_vcs_ignore(true);
        builder.no_ignore(true);
        return;
    }

    let novcs = matches.is_present("no-gitignore");
    builder.no_vcs_ignore(novcs);
    if debug {
        println!(">>> Load Git/VCS ignores: {:?}", !novcs);
    }

    let noignore = matches.is_present("no-ignore");
    builder.no_ignore(noignore);
    if debug {
        println!(">>> Load .ignore ignores: {:?}", !noignore);
    }

    let mut list = vec![
        // Mac
        format!("*{}.DS_Store", MAIN_SEPARATOR),
        // Vim
        "*.sw?".into(),
        "*.sw?x".into(),
        // Emacs
        "#*#".into(),
        ".#*".into(),
        // Kate
        ".*.kate-swp".into(),
        // VCS
        format!("*{s}.hg{s}**", s = MAIN_SEPARATOR),
        format!("*{s}.git{s}**", s = MAIN_SEPARATOR),
        format!("*{s}.svn{s}**", s = MAIN_SEPARATOR),
        // SQLite
        "*.db".into(),
        "*.db-*".into(),
        format!("*{s}*.db-journal{s}**", s = MAIN_SEPARATOR),
        // Rust
        format!("*{s}target{s}**", s = MAIN_SEPARATOR),
    ];

    if debug {
        println!(">>> Default ignores: {:?}", list);
    }

    if matches.is_present("ignore") {
        for ignore in values_t!(matches, "ignore", String).unwrap_or_else(|e| e.exit()) {
            #[cfg(windows)]
            let ignore = ignore.replace("/", &MAIN_SEPARATOR.to_string());
            list.push(ignore);
        }
    }

    if debug {
        println!(">>> All ignores: {:?}", list);
    }

    builder.ignores(list);
}

pub fn set_debounce(debug: bool, builder: &mut ArgsBuilder, matches: &ArgMatches) {
    let d = if matches.is_present("delay") {
        let debounce = value_t!(matches, "delay", f32).unwrap_or_else(|e| e.exit());
        if debug {
            println!(">>> File updates debounce: {} seconds", debounce);
        }
        (debounce * 1000.0) as u32
    } else {
        500
    };

    builder.poll_interval(d).debounce(d);
}

pub fn set_watches(debug: bool, builder: &mut ArgsBuilder, matches: &ArgMatches) {
    let mut opts = Vec::new();
    if matches.is_present("watch") {
        for watch in values_t!(matches, "watch", String).unwrap_or_else(|e| e.exit()) {
            opts.push(watch.into());
        }
    }

    if opts.is_empty() {
        opts.push(".".into());
    }

    if debug {
        println!(">>> Watches: {:?}", opts);
    }

    builder.paths(opts);
}

pub fn get_options(debug: bool, matches: &ArgMatches) -> Args {
    let mut builder = ArgsBuilder::default();
    builder
        .restart(!matches.is_present("no-restart"))
        .poll(matches.is_present("poll"))
        .clear_screen(matches.is_present("clear"))
        .debug(debug)
        .run_initially(!matches.is_present("postpone"));

    set_ignores(debug, &mut builder, &matches);
    set_debounce(debug, &mut builder, &matches);
    set_watches(debug, &mut builder, &matches);
    set_commands(debug, &mut builder, &matches);

    let mut args = builder.build().unwrap();
    args.once = matches.is_present("once");

    if debug {
        println!(">>> Watchexec arguments: {:?}", args);
    }

    args
}
