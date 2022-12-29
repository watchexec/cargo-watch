use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    path::{PathBuf, MAIN_SEPARATOR},
    time::Duration,
};

use cargo_metadata::{MetadataCommand, Node, Package, PackageId};
use clap::{value_t, values_t, ArgMatches};
use log::{debug, warn};
use watchexec::{
    config::{Config, ConfigBuilder},
    run::OnBusyUpdate,
    Shell,
};

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
                        .unwrap_or(cargo.len());

                    // Find returns the byte index, and split_at takes a byte offset.
                    // This means the splitting is unicode-safe.
                    let (subcommand, args) = cargo.split_at(word_boundary);
                    cmd.push_str(subcommand);
                    cmd.push_str(" --features ");
                    cmd.push_str(features);
                    cmd.push(' ');
                    cmd.push_str(args)
                } else {
                    cmd.push_str(cargo);
                }
            } else {
                cmd.push_str(cargo);
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
            cmd.push_str(features);
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

fn find_local_deps() -> Result<Vec<PathBuf>, String> {
    let metadata = MetadataCommand::new()
        .exec()
        .map_err(|e| format!("Failed to execute `cargo metadata`: {}", e))?;

    let resolve = match metadata.resolve {
        None => return Ok(Vec::new()),
        Some(resolve) => resolve,
    };
    let id_to_node =
        HashMap::<PackageId, &Node>::from_iter(resolve.nodes.iter().map(|n| (n.id.clone(), n)));
    let id_to_package = HashMap::<PackageId, &Package>::from_iter(
        metadata.packages.iter().map(|p| (p.id.clone(), p)),
    );

    let mut pkgids_seen = HashSet::new();
    let mut pkgids_to_check = Vec::new();
    match resolve.root {
        Some(root) => pkgids_to_check.push(root),
        None => pkgids_to_check.extend_from_slice(&metadata.workspace_members),
    };

    // The set of directories of all packages we are interested in.
    let mut local_deps = HashSet::new();

    while !pkgids_to_check.is_empty() {
        let current_pkgid = pkgids_to_check.pop().unwrap();
        if !pkgids_seen.insert(current_pkgid.clone()) {
            continue;
        }

        let pkg = match id_to_package.get(&current_pkgid) {
            None => continue,
            Some(&pkg) => pkg,
        };

        // This means this is a remote package. Skip!
        if pkg.source.is_some() {
            continue;
        }

        // This is a path to Cargo.toml.
        let mut path = pkg.manifest_path.clone();
        // We want the directory it's in.
        path.pop();
        local_deps.insert(path.into_std_path_buf());

        // And find dependencies.
        if let Some(node) = id_to_node.get(&current_pkgid) {
            for dep in &node.deps {
                pkgids_to_check.push(dep.pkg.clone());
            }
        }
    }

    Ok(local_deps.into_iter().collect::<Vec<PathBuf>>())
}

pub fn set_watches(builder: &mut ConfigBuilder, matches: &ArgMatches) {
    let mut watches = Vec::new();
    if matches.is_present("watch") {
        for watch in values_t!(matches, "watch", String).unwrap_or_else(|e| e.exit()) {
            watches.push(watch.into());
        }
    }

    if opts.is_empty() && !matches.is_present("skip-local-deps") {
        match find_local_deps() {
            Ok(dirs) => {
                if dirs.is_empty() {
                    debug!("Found no local deps");
                } else {
                    watches = dirs;
                }
            }
            Err(err) => {
                // If this fails just fall back to watching the current directory.
                eprintln!("Finding local deps failed: {}", err);
            }
        }
    }

    if watches.is_empty() {
        watches.push(".".into());
    }

    debug!("Watches: {:?}", watches);
    builder.paths(watches);
}

pub fn get_options(matches: &ArgMatches) -> Config {
    let mut builder = ConfigBuilder::default();
    builder
        .poll(matches.is_present("poll"))
        .clear_screen(matches.is_present("clear"))
        .run_initially(!matches.is_present("postpone"))
        .no_environment(true);

    // TODO in next breaking: remove --watch-when-idle and switch --no-restart behaviour to DoNothing
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
        // in next breaking, just rely on default watchexec behaviour
        default_shell()
    });

    set_ignores(&mut builder, matches);
    set_debounce(&mut builder, matches);
    set_watches(&mut builder, matches);
    set_commands(&mut builder, matches);

    let mut args = builder.build().unwrap();
    args.once = matches.is_present("once");

    debug!("Watchexec arguments: {:?}", args);
    args
}

// until next breaking
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
