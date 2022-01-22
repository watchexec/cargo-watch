pub use init::init;
pub use runtime::runtime;

use crate::args::*;
use clap::Parser;

mod init;
mod runtime;

pub fn get_args() -> Args {
	let args = wild::args_os();
	let args = argfile::expand_args_from(args, argfile::parse_fromfile, argfile::PREFIX).unwrap();

	let app = App::parse_from(args);
	let Command::Watch(args) = app.command;
	args
}
