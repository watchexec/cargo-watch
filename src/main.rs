use std::env::var;

use miette::{IntoDiagnostic, Result};
use watchexec::{
	event::{Event, Priority},
	Watchexec,
};

mod args;
mod config;
// mod filterer;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
	#[cfg(feature = "dev-console")]
	console_subscriber::init();

	if var("RUST_LOG").is_ok() && cfg!(not(feature = "dev-console")) {
		tracing_subscriber::fmt::init();
	}

	let (args, command_order) = config::get_args();

	{
		// TODO
		let verbosity = if args.debug { 2 } else { 0 };
		let mut builder = tracing_subscriber::fmt().with_env_filter(match verbosity {
			0 => "cargo-watch=warn",
			1 => "watchexec=debug,cargo-watch=debug",
			2 => "watchexec=trace,cargo-watch=trace",
			_ => "trace",
		});

		if verbosity > 2 {
			use tracing_subscriber::fmt::format::FmtSpan;
			builder = builder.with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);
		}

		if verbosity > 3 {
			builder.pretty().try_init().ok();
		} else {
			builder.try_init().ok();
		}
	}

	let init = config::init(&args)?;
	let runtime = config::runtime(&args, command_order)?;
	// runtime.filterer(filterer::new(&args).await?);

	let wx = Watchexec::new(init, runtime)?;

	if !args.postpone {
		wx.send_event(Event::default(), Priority::Urgent).await?;
	}

	wx.main().await.into_diagnostic()??;

	Ok(())
}
