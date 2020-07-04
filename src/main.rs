//! Watch files in a Cargo project and compile it when they change
#![warn(clippy::all)]

use cargo_watch::{args, change_dir, watch::CwHandler, get_options};
use stderrlog::Timestamp;
use watchexec::{error::Result, run::watch};

fn main() -> Result<()> {
    let matches = args::parse();
    change_dir();

    let debug = matches.is_present("debug");
    let quiet = matches.is_present("quiet");
    let testing = matches.is_present("once");

    stderrlog::new()
        .quiet(quiet)
        .verbosity(if debug { 3 } else { 1 })
        .timestamp(if testing { Timestamp::Off } else { Timestamp::Millisecond })
        .init()
        .unwrap();

    let opts = get_options(&matches);
    let handler = CwHandler::new(opts, quiet)?;
    watch(&handler)
}
