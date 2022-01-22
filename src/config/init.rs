use std::convert::Infallible;

use clap::ArgMatches;
use miette::Result;
use watchexec::{config::InitConfig, handler::SyncFnHandler};

pub fn init(_args: &ArgMatches<'static>) -> Result<InitConfig> {
	let mut config = InitConfig::default();
	config.on_error(SyncFnHandler::from(
		|data| -> std::result::Result<(), Infallible> {
			if cfg!(debug_assertions) {
				eprintln!("[[{:?}]]", data);
			} else {
				eprintln!("[[{}]]", data);
			}

			Ok(())
		},
	));

	Ok(config)
}
