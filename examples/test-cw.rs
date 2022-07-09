use std::{
	env,
	fs::write,
	path::PathBuf,
	thread::sleep,
	time::{Duration, Instant},
};

use clap::Parser;
use duct::cmd;
use miette::{bail, IntoDiagnostic, Result};
use tempfile::TempDir;

#[derive(Debug, Parser)]
struct Args {
	#[clap(long)]
	vcs: Option<String>,

	#[clap(long)]
	bin: bool,

	#[clap(long, multiple_occurrences = true)]
	touch: Vec<PathBuf>,

	#[clap(long, default_value = "200")]
	before: u64,

	#[clap(long, default_value = "200")]
	between: u64,

	#[clap(long, default_value = "5000")]
	timeout: u64,

	#[clap(long)]
	expect_timeout: bool,

	#[clap(raw = true)]
	cmd: Vec<String>,
}

fn main() -> Result<()> {
	let mut args = Args::parse();

	let tmp_dir = TempDir::new_in(".").into_diagnostic()?;

	let mut init_args = vec!["init", "--quiet", "--name", "cw-test", "--offline"];
	if args.bin {
		init_args.push("--bin");
	}
	if let Some(vcs) = &args.vcs {
		init_args.push("--vcs");
		init_args.push(vcs);
	}
	cmd("cargo", init_args)
		.dir(tmp_dir.path())
		.run()
		.into_diagnostic()?;

	let before = Duration::from_millis(args.before);
	let between = Duration::from_millis(args.between);
	let timeout = Duration::from_millis(args.timeout);

	args.cmd.insert(0, "watch".into());
	let cw = cmd(
		PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target/debug/cargo-watch"),
		args.cmd,
	)
	.dir(tmp_dir.path())
	.start()
	.into_diagnostic()?;

	sleep(before);

	for file in args.touch {
		write(tmp_dir.path().join(file), "something").into_diagnostic()?;
		sleep(between);
	}

	let start = Instant::now();
	let mut timed_out = true;
	let mut output = None;
	while start.elapsed() < timeout {
		if let Some(out) = cw.try_wait().into_diagnostic()? {
			timed_out = false;
			output.replace(out);
			break;
		} else {
			sleep(Duration::from_millis(10));
		}
	}

	cw.kill().into_diagnostic()?;
	tmp_dir.close().into_diagnostic()?;

	match (timed_out, args.expect_timeout) {
		(true, false) => bail!("Timed out"),
		(false, false) => {}
		(true, true) => eprintln!("{{~expected timeout~}}"),
		(false, true) => bail!("Expected timeout, but got quit: {:?}", output),
	}

	Ok(())
}
