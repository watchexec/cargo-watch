//! Watch files in a Cargo project and compile it when they change

extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
#[cfg(not(windows))]
extern crate libc;
#[macro_use]
extern crate log;
extern crate notify;
extern crate regex;
extern crate rustc_serialize;
extern crate wait_timeout;

use docopt::Docopt;
use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;

mod cargo;
mod config;
mod schedule;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static USAGE: &'static str = r#"
Usage: cargo-watch [watch] [options]
       cargo watch [options]
       cargo-watch [watch] [<args>...]
       cargo watch [<args>...]

Options:
  -h, --help      Display this message
  --version       Show version

`cargo watch` can take one or more arguments to pass to cargo. For example,
`cargo watch "test ex_ --release"` will run `cargo test ex_ --release`

If no arguments are provided, then cargo will run `build` and `test`
"#;

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_args: Vec<String>,
    flag_version: bool,
}

fn main() {
    // Initialize logger
    env_logger::init().unwrap();

    // Parse CLI parameters
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("cargo-watch {}", VERSION);
        std::process::exit(0);
    }

    let commands = args.arg_args;

    // Check if we are (somewhere) in a cargo project directory
    let cargo_dir = match cargo::root() {
        Some(path) => path,
        None => {
            error!("Not a Cargo project, aborting.");
            std::process::exit(64);
        },
    };

    // Creates `Watcher` instance and a channel to communicate with it
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = match Watcher::new(tx) {
        Ok(i) => i,
        Err(e) => {
            error!("Failed to init notify ({:?})", e);
            std::process::exit(1);
        },
    };

    // Configure watcher: we want to watch these subfolders
    for subdir in &config::WATCH_DIRS {
        // We ignore any errors (e.g. if the directory doesn't exist)
        let _ = watcher.watch(&cargo_dir.join(subdir));
    }

    // Tell the user that we are ready
    println!("Waiting for changes... Hit Ctrl-C to stop.");

    // Handle incoming events from the watcher
    schedule::handle(rx, commands);
}
