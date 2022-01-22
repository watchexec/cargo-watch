use std::env::var;

use miette::{IntoDiagnostic, Result};
use watchexec::{event::Event, Watchexec};

mod args;
mod config;
// mod filterer;

#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
	#[cfg(feature = "dev-console")]
	console_subscriber::init();

	if var("RUST_LOG").is_ok() && cfg!(not(feature = "dev-console")) {
		tracing_subscriber::fmt::init();
	}

	let args = args::get_args()?;

	{
		let verbosity = args.occurrences_of("verbose");
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
	let runtime = config::runtime(&args)?;
	// runtime.filterer(filterer::new(&args).await?);

	let wx = Watchexec::new(init, runtime)?;

	if !args.is_present("postpone") {
		wx.send_event(Event::default()).await?;
	}

	wx.main().await.into_diagnostic()??;

	Ok(())
}
