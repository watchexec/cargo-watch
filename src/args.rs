use std::{ffi::OsString, path::PathBuf};

use clap::Parser;

const OPTSET_FILTERING: &str = "FILTERING";
const OPTSET_COMMAND: &str = "COMMAND";
const OPTSET_ENVIRONMENT: &str = "ENVIRONMENT";
const OPTSET_DEBUGGING: &str = "DEBUGGING";
const OPTSET_OUTPUT: &str = "OUTPUT";
const OPTSET_BEHAVIOUR: &str = "BEHAVIOUR";
const OPTSET_WORKSPACES: &str = "WORKSPACES";

#[derive(Debug, Clone, Parser)]
#[clap(name = "cargo-watch", bin_name = "cargo", version)]
pub struct App {
	#[clap(subcommand)]
	pub command: Command,
}

#[derive(Debug, Clone, clap::Subcommand)]
#[clap(name = "cargo-watch", bin_name = "cargo", version)]
pub enum Command {
	#[clap(name = "watch")]
	Watch(Args),
}

// --watch-when-idle is now default

#[derive(Debug, Clone, clap::Args)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	#[clap(hide = true, long = "testing-only--once")]
	pub once: bool,

	/// Clear the screen before each run
	#[clap(short = 'c', long = "clear", help_heading = OPTSET_OUTPUT)]
	pub clear: bool,

	/// Show debug output
	#[clap(long = "debug", help_heading = OPTSET_DEBUGGING)]
	pub debug: bool,

	/// Show paths that changed
	#[clap(long = "why", help_heading = OPTSET_DEBUGGING)]
	pub why: bool,

	/// Ignore nothing, not even target/ and .git/
	#[clap(long = "ignore-nothing", help_heading = OPTSET_FILTERING)]
	pub ignore_nothing: bool,

	/// Don’t use .gitignore files
	#[clap(long = "no-gitignore", help_heading = OPTSET_FILTERING)]
	pub no_gitignore: bool,

	/// Don’t use .ignore files
	#[clap(long = "no-ignore", help_heading = OPTSET_FILTERING)]
	pub no_ignore: bool,

	/// Don’t restart command while it’s still running
	#[clap(long = "no-restart", help_heading = OPTSET_BEHAVIOUR)]
	pub no_restart: bool,

	/// Reserves for workspace support
	#[clap(long = "all", hide = true, help_heading = OPTSET_WORKSPACES)]
	pub packages_all: bool,

	/// Force use of polling for file changes
	#[clap(long = "poll", help_heading = OPTSET_BEHAVIOUR)]
	pub poll: bool,

	/// Postpone first run until a file changes
	#[clap(long = "postpone", help_heading = OPTSET_BEHAVIOUR)]
	pub postpone: bool,

	/// Feature(s) passed to cargo invocations
	///
	/// This is passed to cargo commands specified with `-x` only, and
	/// which start with `b`, `check`, `doc`, `r`, `test`, or `install`.
	#[clap(long = "features", help_heading = OPTSET_COMMAND)]
	pub features: Vec<String>,

	/// Suppress output from cargo watch itself
	///
	/// By default, cargo watch will print a message to stderr when the
	/// command starts and finishes.
	#[clap(short = 'q', long = "quiet", help_heading = OPTSET_OUTPUT)]
	pub quiet: bool,

	/// Cargo command(s) to execute on changes
	///
	/// By default, `cargo check` is run.
	#[clap(
		short = 'x',
		long = "exec",
		multiple_occurrences = true,
		value_name = "cmd",
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		help_heading = OPTSET_COMMAND
	)]
	pub cmd_cargo: Vec<String>,

	/// Shell command(s) to execute on changes
	///
	/// This may not necessarily be run in a shell, e.g. with
	/// `--use-shell=none`.
	#[clap(
		short = 's',
		long = "shell",
		multiple_occurrences = true,
		value_name = "cmd",
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		help_heading = OPTSET_COMMAND
	)]
	pub cmd_shell: Vec<String>,

	/// File updates debounce delay
	///
	/// During this time, incoming change events are accumulated and
	/// only once the delay has passed, is an action taken. Note that
	/// this does not mean a command will be started: if --no-restart is
	/// given and a command is already running, the outcome of the
	/// action will be to do nothing.
	///
	/// Defaults to 50ms. Parses as decimal seconds by default, but
	/// using an integer with the `ms` suffix may be more convenient.
	/// When using --poll mode, you'll want a larger duration, or risk
	/// overloading disk I/O.
	#[clap(
		short = 'd',
		long = "delay",
		forbid_empty_values = true,
		help_heading = OPTSET_BEHAVIOUR
	)]
	pub delay: Option<String>,

	/// Ignore a path pattern
	///
	/// This is in gitignore or glob format. Use a leading `!` for
	/// allowlisting.
	#[clap(
		short = 'i',
		long = "ignore",
		value_name = "pattern",
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		help_heading = OPTSET_FILTERING
	)]
	pub ignores: Vec<String>,

	/// Reserved for workspace support
	#[clap(
		short = 'p',
		long = "package",
		value_name = "spec",
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		hide = true,
		help_heading = OPTSET_WORKSPACES
	)]
	pub packages_specs: Vec<String>,

	/// Watch specific file(s) or folder(s)
	///
	/// By default, the entire crate/workspace is watched.
	#[clap(
		short = 'w',
		long = "watch",
		value_name = "path",
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		help_heading = OPTSET_FILTERING
	)]
	pub watch: Vec<PathBuf>,

	/// Shell to use for --shell commands, or `none` for direct execution.
	///
	/// This applies only to `--shell` / `-s` commands, `--exec` / `-x` cargo commands are
	/// executed directly, without a shell. The option applies to all subsequent shell
	/// commands:
	///
	///     cargo watch --use-shell=zsh -s one -s two
	///
	/// will use zsh for commands one and two, but:
	///
	///     cargo watch -s one --use-shell=zsh -s two
	///
	/// will only use zsh for the second one.
	///
	/// As a convenience, if only one --use-shell is provided and it is used after all command
	/// arguments, it is interpreted as if it was given first:
	///
	///     cargo watch -s one -s two --use-shell=zsh
	///
	/// will run both one and two with zsh. (Otherwise the option would do nothing.)
	///
	/// The first word must be the shell program, but it can be followed by options to pass to
	/// the shell program:
	///
	///     cargo watch --use-shell='bash -s globext' -- 'ls **'
	///
	/// On Windows, defaults to Powershell. Elsewhere, defaults to $SHELL, falling back to `sh`
	/// if not available.
	// TODO: check that the bash shopt makes any sense
	// TODO: check that clap doesn't eat all this nice layouting
	#[clap(long = "use-shell", value_name = "shell", help_heading = OPTSET_ENVIRONMENT,
		multiple_occurrences = true,
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
            )]
	pub use_shell: Vec<String>,

	/// Change working directory of the command
	///
	/// This defaults to the crate or workspace root.
	#[clap(short = 'C', long = "workdir", value_name = "path", help_heading = OPTSET_ENVIRONMENT)]
	pub workdir: Option<PathBuf>,

	/// Send a desktop notification on command start and end
	///
	/// The message will include success or failure, with the exit code
	/// returned by the command.
	#[clap(short = 'N', long = "notify", help_heading = OPTSET_OUTPUT)]
	#[cfg_attr(target_os = "freebsd", clap(hide = true))]
	pub notif: bool,

	/// Inject RUST_BACKTRACE=value into the command's environment
	///
	/// Examples: -B=1, -B=full
	#[clap(short = 'B', help_heading = OPTSET_ENVIRONMENT)]
	pub env_backtrace: bool,

	/// Inject RUST_LOG=value into the command's environment
	///
	/// Examples: -L=debug, -L=info,cratename::module=debug
	#[clap(short = 'L', help_heading = OPTSET_ENVIRONMENT)]
	pub env_log: bool,

	/// Full command to run. -x and -s will be ignored!
	#[clap(raw = true, value_name = "trailing command", help_heading = OPTSET_COMMAND)]
	pub cmd_trail: Option<OsString>,
}
