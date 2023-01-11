use std::path::PathBuf;

use camino::Utf8PathBuf;
use clap::values_t;
use stderrlog::Timestamp;
use watchexec::{error::Result, run::watch};

mod args;
mod options;
mod root;
mod watch;

fn main() -> Result<()> {
    let matches = args::parse();

    let debug = matches.is_present("log:debug");
    let info = matches.is_present("log:info");
    let quiet = matches.is_present("log:quiet");
    let testing = matches.is_present("once");

    stderrlog::new()
        .quiet(quiet)
        .show_module_names(debug)
        .verbosity(if debug {
            3
        } else if info {
            2
        } else {
            1
        })
        .timestamp(if testing {
            Timestamp::Off
        } else {
            Timestamp::Millisecond
        })
        .init()
        .unwrap();

    root::change_dir(
        matches
            .value_of("workdir")
            .map(Utf8PathBuf::from)
            .unwrap_or_else(root::project_root),
    );

    if let Some(b) = matches.value_of("rust-backtrace") {
        // Soundness: not great, it'll get better with watchexec 2
        std::env::set_var("RUST_BACKTRACE", b);
    }

    if let Some(l) = matches.value_of("rust-log") {
        // Soundness: not great, it'll get better with watchexec 2
        std::env::set_var("RUST_LOG", l);
    }

    if matches.is_present("env-files") {
        for file in values_t!(matches, "env-files", PathBuf).unwrap_or_else(|e| e.exit()) {
            for item in dotenvy::from_path_iter(&file).unwrap_or_else(|e| {
                clap::Error::with_description(
                    &format!("Failed to read .env file {file:?}: {e}"),
                    clap::ErrorKind::ValueValidation,
                )
                .exit()
            }) {
                let (key, var) = item.unwrap_or_else(|e| {
                    clap::Error::with_description(
                        &format!("Malformed pair in .env file {file:?}: {e}"),
                        clap::ErrorKind::ValueValidation,
                    )
                    .exit()
                });
                // Soundness: not great, it'll get better with watchexec 2
                std::env::set_var(key, var);
            }
        }
    }

    if matches.is_present("env-vars") {
        for pair in values_t!(matches, "env-vars", String).unwrap_or_else(|e| e.exit()) {
            if let Some((key, var)) = pair.split_once('=') {
                // Soundness: not great, it'll get better with watchexec 2
                std::env::set_var(key, var);
            } else {
                eprintln!("Malformed environment variable '{pair}', ignoring");
            }
        }
    }

    let opts = options::get_options(&matches);
    let handler = watch::CwHandler::new(
        opts,
        quiet,
        matches.is_present("notif"),
        matches.is_present("cmd:trail"),
    )?;
    watch(&handler)
}
