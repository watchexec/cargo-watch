//! Watch files in a Cargo project and compile it when they change
#![forbid(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy_pedantic))]

extern crate cargo_watch;
extern crate watchexec;

fn main() -> watchexec::error::Result<()> {
    let matches = cargo_watch::args::parse();

    cargo_watch::change_dir();

    let debug = matches.is_present("debug");
    let opts = cargo_watch::get_options(debug, &matches);
    watchexec::run::watch::<cargo_watch::watch::CwHandler>(opts)
}
