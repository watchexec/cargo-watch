pub use init::init;
pub use runtime::runtime;

use crate::args::*;
use clap::Parser;

mod init;
mod runtime;

pub fn get_args() -> (Args, Vec<&'static str>) {
	let args = wild::args_os();
	let args = argfile::expand_args_from(args, argfile::parse_fromfile, argfile::PREFIX).unwrap();

	let command_order = args
		.iter()
		.filter_map(|arg| match arg.to_str() {
			Some("-x" | "--exec") => Some("cargo"),
			Some("-s" | "--shell") => Some("shell"),
			Some("--use-shell") => Some("use-shell"),
			_ => None,
		})
		.collect::<Vec<_>>();

	let app = App::parse_from(args);
	let Command::Watch(args) = app.command;
	(args, command_order)
}
