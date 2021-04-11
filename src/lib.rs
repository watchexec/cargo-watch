//! Watch files in a Cargo project and compile it when they change
#![warn(clippy::all)]

use clap::{value_t, values_t, ArgMatches, Error, ErrorKind};
use log::{debug, warn};
use std::{env::set_current_dir, path::MAIN_SEPARATOR, time::Duration};
use watchexec::{
    config::{Config, ConfigBuilder},
    run::OnBusyUpdate,
    Shell,
};

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

pub fn set_commands(builder: &mut ConfigBuilder, matches: &ArgMatches) {
    let mut commands: Vec<String> = Vec::new();

    // --features are injected just after applicable cargo subcommands
    // and before the remaining arguments
    let features = value_t!(matches, "features", String).ok();

    // Cargo commands are in front of the rest
    if matches.is_present("cmd:cargo") {
        for cargo in values_t!(matches, "cmd:cargo", String).unwrap_or_else(|e| e.exit()) {
            let mut cmd: String = "cargo ".into();
            let cargo = cargo.trim_start();
            // features are supported for the following
            // (b)uild, bench, doc, (r)un, test, install
            if let Some(features) = features.as_ref() {
                if cargo.starts_with('b')
                    || cargo.starts_with("check")
                    || cargo.starts_with("doc")
                    || cargo.starts_with('r')
                    || cargo.starts_with("test")
                    || cargo.starts_with("install")
                {
                    // Split command into first word and the arguments
                    let word_boundary = cargo
                        .find(|c: char| c.is_whitespace())
                        .unwrap_or_else(|| cargo.len());

                    // Find returns the byte index, and split_at takes a byte offset.
                    // This means the splitting is unicode-safe.
                    let (subcommand, args) = cargo.split_at(word_boundary);
                    cmd.push_str(subcommand);
                    cmd.push_str(" --features ");
                    cmd.push_str(features);
                    cmd.push(' ');
                    cmd.push_str(args)
                } else {
                    cmd.push_str(&cargo);
                }
            } else {
                cmd.push_str(&cargo);
            }
            commands.push(cmd);
        }
    }

    // Shell commands go last
    if matches.is_present("cmd:shell") {
        for shell in values_t!(matches, "cmd:shell", String).unwrap_or_else(|e| e.exit()) {
            commands.push(shell);
        }
    }

    if matches.is_present("cmd:trail") {
        debug!("trailing command is present, ignore all other command options");
        commands = vec![values_t!(matches, "cmd:trail", String)
            .unwrap_or_else(|e| e.exit())
            .into_iter()
            .map(|arg| shell_escape::escape(arg.into()))
            .collect::<Vec<_>>()
            .join(" ")];
    }

    // Default to `cargo check`
    if commands.is_empty() {
        let mut cmd: String = "cargo check".into();
        if let Some(features) = features.as_ref() {
            cmd.push_str(" --features ");
            cmd.push_str(&features);
        }
        commands.push(cmd);
    }

    debug!("Commands: {:?}", commands);
    builder.cmd(commands);
}

pub fn set_ignores(builder: &mut ConfigBuilder, matches: &ArgMatches) {
    if matches.is_present("ignore-nothing") {
        debug!("Ignoring nothing");

        builder.no_vcs_ignore(true);
        builder.no_ignore(true);
        return;
    }

    let novcs = matches.is_present("no-gitignore");
    builder.no_vcs_ignore(novcs);
    debug!("Load Git/VCS ignores: {:?}", !novcs);

    let noignore = matches.is_present("no-ignore");
    builder.no_ignore(noignore);
    debug!("Load .ignore ignores: {:?}", !noignore);

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

    debug!("Default ignores: {:?}", list);

    if matches.is_present("ignore") {
        for ignore in values_t!(matches, "ignore", String).unwrap_or_else(|e| e.exit()) {
            #[cfg(windows)]
            let ignore = ignore.replace("/", &MAIN_SEPARATOR.to_string());
            list.push(ignore);
        }
    }

    debug!("All ignores: {:?}", list);
    builder.ignores(list);
}

pub fn set_debounce(builder: &mut ConfigBuilder, matches: &ArgMatches) {
    if matches.is_present("delay") {
        let debounce = value_t!(matches, "delay", f32).unwrap_or_else(|e| e.exit());
        debug!("File updates debounce: {} seconds", debounce);

        let d = Duration::from_millis((debounce * 1000.0) as u64);
        builder.poll_interval(d).debounce(d);
    }
}

pub fn set_watches(builder: &mut ConfigBuilder, matches: &ArgMatches) {
    let mut opts = Vec::new();
    if matches.is_present("watch") {
        for watch in values_t!(matches, "watch", String).unwrap_or_else(|e| e.exit()) {
            opts.push(watch.into());
        }
    }

    if opts.is_empty() {
        opts.push(".".into());
    }

    debug!("Watches: {:?}", opts);
    builder.paths(opts);
}

pub fn get_options(matches: &ArgMatches) -> Config {
    let mut builder = ConfigBuilder::default();
    builder
        .poll(matches.is_present("poll"))
        .clear_screen(matches.is_present("clear"))
        .run_initially(!matches.is_present("postpone"))
        .no_environment(true);

    // TODO in 8.0: remove --watch-when-idle and switch --no-restart behaviour to DoNothing
    builder.on_busy_update(if matches.is_present("no-restart") {
        OnBusyUpdate::Queue
    } else if matches.is_present("watch-when-idle") {
        OnBusyUpdate::DoNothing
    } else {
        OnBusyUpdate::Restart
    });

    builder.shell(if let Some(s) = matches.value_of("use-shell") {
        if s.eq_ignore_ascii_case("powershell") {
            Shell::Powershell
        } else if s.eq_ignore_ascii_case("none") {
            warn!("--use-shell=none is non-sensical for cargo-watch, ignoring");
            default_shell()
        } else if s.eq_ignore_ascii_case("cmd") {
            cmd_shell(s.into())
        } else {
            Shell::Unix(s.into())
        }
    } else {
        // in 8.0, just rely on default watchexec behaviour
        default_shell()
    });

    set_ignores(&mut builder, &matches);
    set_debounce(&mut builder, &matches);
    set_watches(&mut builder, &matches);
    set_commands(&mut builder, &matches);

    let mut args = builder.build().unwrap();
    args.once = matches.is_present("once");

    debug!("Watchexec arguments: {:?}", args);
    args
}

// until 8.0
#[cfg(windows)]
fn default_shell() -> Shell {
    Shell::Cmd
}

#[cfg(not(windows))]
fn default_shell() -> Shell {
    Shell::default()
}

// because Shell::Cmd is only on windows
#[cfg(windows)]
fn cmd_shell(_: String) -> Shell {
    Shell::Cmd
}

#[cfg(not(windows))]
fn cmd_shell(s: String) -> Shell {
    Shell::Unix(s)
}
