use stderrlog::Timestamp;
use watchexec::{error::Result, run::watch};

use options::get_options;
use root::change_dir;
use watch::CwHandler;

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

    change_dir();
    let opts = get_options(&matches);
    let handler = CwHandler::new(opts, quiet)?;
    watch(&handler)
}
