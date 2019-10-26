//! Watch files in a Cargo project and compile it when they change
#![forbid(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy_pedantic))]

extern crate cargo_watch;
extern crate watchexec;

fn main() -> watchexec::error::Result<()> {
    let matches = cargo_watch::args::parse();

    cargo_watch::change_dir();

    let quiet = matches.is_present("quiet");
    let debug = matches.is_present("debug");
    let mut watchexec_args = cargo_watch::get_watchexec_args(debug, &matches);

    let handler = cargo_watch::watch::CwHandler::new(&mut watchexec_args, quiet)?;

    watchexec::run::watch(&handler)
}
