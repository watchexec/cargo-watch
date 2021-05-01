use camino::Utf8PathBuf;
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

    let opts = options::get_options(&matches);
    let handler = watch::CwHandler::new(opts, quiet)?;
    watch(&handler)
}
