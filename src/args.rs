use std::{ffi::OsString, path::PathBuf};

use clap::Parser;

const OPTSET_FILTERING: &str = "Filtering options:";
const OPTSET_COMMAND: &str = "Command options:";
const OPTSET_DEBUGGING: &str = "Debugging options:";
const OPTSET_OUTPUT: &str = "Output options:";
const OPTSET_BEHAVIOUR: &str = "Behaviour options:";

#[derive(Debug, Clone, Parser)]
#[clap(name = "cargo-watch", bin_name = "cargo", version)]
struct App {
	#[clap(subcommand)]
	command: Command,
}

#[derive(Debug, Clone, clap::Subcommand)]
#[clap(name = "cargo-watch", bin_name = "cargo", version)]
enum Command {
	#[clap(name = "watch")]
	Watch(Args),
}

#[derive(Debug, Clone, clap::Args)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	#[clap(hide = true, long = "testing-only--once")]
	pub once: bool,

	/// Clear the screen before each run
	#[clap(short = 'c', long = "clear")]
	pub clear: bool,

	/// Show debug output
	#[clap(long = "debug")]
	pub debug: bool,

	/// Show paths that changed
	#[clap(long = "why")]
	pub why: bool,

	/// Ignore nothing, not even target/ and .git/
	#[clap(long = "ignore-nothing")]
	pub ignore_nothing: bool,

	/// Don’t use .gitignore files
	#[clap(long = "no-gitignore")]
	pub no_gitignore: bool,

	/// Don’t use .ignore files
	#[clap(long = "no-ignore")]
	pub no_ignore: bool,

	/// Don’t restart command while it’s still running
	#[clap(long = "no-restart")]
	pub no_restart: bool,

	/// Reserves for workspace support
	#[clap(long = "all", hide = true)]
	pub packages_all: bool,

	/// Force use of polling for file changes
	#[clap(long = "poll")]
	pub poll: bool,

	/// Postpone first run until a file changes
	#[clap(long = "postpone")]
	pub postpone: bool,

	// --watch-when-idle is now default
	/// List of features passed to cargo invocations
	#[clap(long = "features")]
	pub features: Vec<String>,

	/// Suppress output from cargo-watch itself
	#[clap(short = 'q', long = "quiet")]
	pub quiet: bool,

	/// Cargo command(s) to execute on changes [default: check]
	#[clap(
		short = 'x',
		long = "exec",
		multiple_occurrences = true,
		value_name = "cmd",
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1
	)]
	pub cmd_cargo: Vec<String>,

	/// Shell command(s) to execute on changes
	#[clap(
		short = 's',
		long = "shell",
		multiple_occurrences = true,
		value_name = "cmd",
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1
	)]
	pub cmd_shell: Vec<String>,

	/// File updates debounce delay in seconds
	#[clap(
		short = 'd',
		long = "delay",
		default_value = "0.05",
		forbid_empty_values = true
	)]
	pub delay: f32,

	/// Ignore a glob/gitignore-style pattern
	#[clap(
		short = 'i',
		long = "ignore",
		value_name = "pattern",
		multiple_occurrences = true,
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1
	)]
	pub ignores: Vec<String>,

	/// Reserved for workspace support
	#[clap(
		short = 'p',
		long = "package",
		value_name = "spec",
		multiple_occurrences = true,
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		hide = true
	)]
	pub packages_specs: Vec<String>,

	/// Watch specific file(s) or folder(s)
	#[clap(
		short = 'w',
		long = "watch",
		value_name = "path",
		multiple_occurrences = true,
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		default_value = "."
	)]
	pub watch: Vec<PathBuf>,

	/// Shell to use
	#[cfg_attr(
		windows,
		clap(
			help = "Use a different shell. Defaults to Powershell, use --use-shell=cmd to use cmd.exe"
		)
	)]
	#[cfg_attr(
		unix,
		clap(help = "Use a different shell. Defaults to $SHELL. E.g. --use-shell=sh")
	)]
	#[clap(long = "use-shell", value_name = "shell")]
	pub shell: Option<String>,

	/// Change working directory before running shell command [default: crate root]
	#[clap(short = 'C', long = "workdir", value_name = "path")]
	pub workdir: Option<PathBuf>,

	/// Send a desktop notification when watchexec starts and ends the command
	#[clap(short = 'N', long = "notify")]
	#[cfg_attr(target_os = "freebsd", clap(hide = true))]
	pub notif: bool,

	/// Inject RUST_BACKTRACE=value (generally you want to set it to 1) into the command's environment
	#[clap(short = 'B')]
	pub env_backtrace: bool,

	/// Inject RUST_LOG=value (e.g. debug, trace) into the command's environment
	#[clap(short = 'L')]
	pub env_log: bool,

	/// Full command to run. -x and -s will be ignored!
	#[clap(raw = true)]
	pub cmd_trail: Option<OsString>,
}

pub fn get_args() -> Args {
	let args = wild::args_os();
	let args = argfile::expand_args_from(args, argfile::parse_fromfile, argfile::PREFIX).unwrap();

	let app = App::parse_from(args);
	let Command::Watch(args) = app.command;
	args
}
