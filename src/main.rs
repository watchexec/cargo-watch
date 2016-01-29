//! Watch files in a Cargo project and compile it when they change

extern crate rustc_serialize;
extern crate docopt;
extern crate notify;
#[macro_use]
extern crate log;
extern crate env_logger;

use docopt::Docopt;
use notify::{Error, RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::sync::Arc;

mod cargo;
mod compile;
mod ignore;
mod timelock;

static USAGE: &'static str = r#"
Usage: cargo-watch [watch] [options]
       cargo watch [options]
       cargo-watch [watch] [<args>...]
       cargo watch [<args>...]

Options:
  -h, --help      Display this message

`cargo watch` can take one or more arguments to pass to cargo. For example,
`cargo watch "test ex_ --release"` will run `cargo test ex_ --release`

If no arguments are provided, then cargo will run `build` and `test`
"#;

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_args: Vec<String>,
}

#[derive(Clone)]
pub struct Config {
    args: Vec<String>,
}

impl Config {
    fn new() -> Config {
        let Args { arg_args: args } = Docopt::new(USAGE)
                                             .and_then(|d| d.decode())
                                             .unwrap_or_else(|e| e.exit());

        Config { args: args }
    }
}

fn main() {
    // Initialize logger functionality
    env_logger::init().unwrap();

    // Read and save CLI parameters
    let config = Config::new();

    // Creates `Watcher` instance to watch some folders
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = match Watcher::new(tx) {
        Ok(i) => i,
        Err(_) => {
            error!("Failed to init notify");
            std::process::exit(1);
        }
    };

    let t = timelock::new();
    let c = Arc::new(config);

    // Check if we are (somewhere) in a cargo project directory
    match cargo::root() {
        Some(p) => {
            // We want to watch these subfolders
            let _ = watcher.watch(&p.join("src"));
            let _ = watcher.watch(&p.join("tests"));
            let _ = watcher.watch(&p.join("benches"));

            println!("Waiting for changes... Hit Ctrl-C to stop.");

            // Handle events as long as the watcher still sends events
            // through the channel
            while let Ok(event) = rx.recv() {
                compile::handle_event(&t, event, c.clone());
            }
        }
        None => {
            error!("Not a Cargo project, aborting.");
            std::process::exit(64);
        }
    }
}
