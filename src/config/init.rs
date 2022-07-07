use std::convert::Infallible;

use miette::Result;
use watchexec::{config::InitConfig, handler::SyncFnHandler};

use crate::args::Args;

pub fn init(_args: &Args) -> Result<InitConfig> {
	let mut config = InitConfig::default();
	config.on_error(SyncFnHandler::from(
		|data| -> std::result::Result<(), Infallible> {
			eprintln!("[[{:?}]]", data);
			Ok(())
		},
	));

	Ok(config)
}
