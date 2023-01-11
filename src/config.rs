use argfile::{expand_args_from, parse_fromfile};
use clap::Parser;
use tracing::info;

use crate::args::*;
pub use init::init;
pub use runtime::runtime;

mod init;
//mod origin;
mod runtime;

pub fn get_args() -> (Args, Vec<&'static str>) {
	let args = wild::args_os();
	let mut args = expand_args_from(args, parse_fromfile, argfile::PREFIX).unwrap();

	// Filter extraneous arg when invoked by cargo
	// `cargo-watch` gives ["/path/to/cargo-watch"]
	// `cargo watch` gives ["/path/to/cargo-watch", "watch"]
	if args.len() > 1 && args[1] == "watch" {
		args.remove(1);
	}

	let command_order = args
		.iter()
		.filter_map(|arg| match arg.to_str() {
			Some("-x" | "--exec") => Some("cargo"),
			Some("-s" | "--shell") => Some("shell"),
			Some("--use-shell") => Some("use-shell"),
			_ => None,
		})
		.collect::<Vec<_>>();

	info!(?args, "arguments before parsing");
	let args = Args::parse_from(args);
	info!(?args, ?command_order, "arguments parsed");
	(args, command_order)
}
