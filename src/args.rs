use std::path::PathBuf;

const OPTSET_FILTERING: &str = "FILTERING";
const OPTSET_COMMAND: &str = "COMMAND";
const OPTSET_ENVIRONMENT: &str = "ENVIRONMENT";
const OPTSET_DEBUGGING: &str = "DEBUGGING";
const OPTSET_OUTPUT: &str = "OUTPUT";
const OPTSET_BEHAVIOUR: &str = "BEHAVIOUR";

#[derive(Debug, Clone, clap::Parser)]
#[clap(name = "cargo-watch", about, version)]
pub struct Args {
	/// Show the help.
	#[clap(
		short = 'h',
		long,
		help_heading = OPTSET_DEBUGGING,
	)]
	pub help: bool,

	/// Show the version.
	#[clap(
		short = 'V',
		long,
		help_heading = OPTSET_DEBUGGING,
	)]
	pub version: bool,

	/// Clear the screen before each run.
	///
	/// This does a hard clear (clear the screen and the backlog), or as is supported by your
	/// terminal (some terminals only allow soft clears, ie without erasing the backlog).
	///
	/// Use --reset to be more thorough.
	#[clap(
		short = 'c',
		long,
		conflicts_with = "reset",
		help_heading = OPTSET_OUTPUT,
	)]
	pub clear: bool,

	/// Reset the screen before each run.
	///
	/// This does a full terminal reset, including a clear, setting "cooked" mode, and leaving the
	/// alternative buffer.
	///
	/// TUI applications such as editors and pagers often set raw mode to gain precise control of
	/// the terminal state. If such a program crashes, it may not reset the terminal mode back to
	/// the mode it found it in, which can leave the terminal behaving oddly or rendering it
	/// completely unusable.
	///
	/// This uses the clearscreen library's "WellDone" mode on unix, which is more thorough than
	/// many reset implementations.
	#[clap(
		long,
		conflicts_with = "clear",
		help_heading = OPTSET_OUTPUT,
	)]
	pub reset: bool,

	/// Show debug output.
	///
	/// This is extremely verbose. Consider using `RUST_LOG` to restrict output, and/or
	/// --debug-file to send logs to a file instead of stderr.
	#[clap(
		long,
		help_heading = OPTSET_DEBUGGING,
	)]
	pub debug: bool,

	/// Send debug output to a file instead of stderr.
	///
	/// This implies --debug.
	///
	/// You should specify a path outside the watched area, or ignore the resulting logfile,
	/// otherwise you'll loop Cargo Watch.
	#[clap(
		long,
		help_heading = OPTSET_DEBUGGING,
	)]
	pub debug_file: Option<PathBuf>,

	/// Show events that triggered a run.
	#[clap(
		long = "why",
		help_heading = OPTSET_DEBUGGING,
	)]
	pub print_events: bool,

	/// Suppress output from cargo watch itself.
	///
	/// By default, cargo watch will print a message to stderr when the command starts and
	/// finishes.
	#[clap(
		short = 'q',
		long,
		help_heading = OPTSET_OUTPUT,
	)]
	pub quiet: bool,

	/// Ignore nothing, not even target/ and .git/
	#[clap(
		long,
		help_heading = OPTSET_FILTERING,
	)]
	pub ignore_nothing: bool,

	/// Don’t use or look for VCS ignore files.
	#[clap(
		long,
		help_heading = OPTSET_FILTERING,
	)]
	pub no_vcs_ignores: bool,

	/// Override the VCS origin.
	///
	/// The VCS origin is used to resolve ignore files and as the root of ignore patterns. From this
	/// origin, ignore files for the VCS in use will be discovered in descendant directories.
	///
	/// By default this is not necessarily the root of the Cargo workspace, but can look up to the
	/// root of the Git repository, for example. In some cases, this can be overzealous or too
	/// broad, and this option lets you override it.
	///
	/// However, do consider filing a bug if you think the default resolution could be improved.
	///
	/// Use --no-vcs-ignores to disable both using VCS ignore files and discovering an origin.
	#[clap(
		long,
		value_name = "path",
		help_heading = OPTSET_FILTERING,
	)]
	pub vcs_origin: Option<PathBuf>,

	/// Don’t use .ignore files
	#[clap(
		long,
		help_heading = OPTSET_FILTERING,
	)]
	pub no_dot_ignores: bool,

	/// Restart the command set when events come in while it’s still running.
	///
	/// Note that this can lead to loops when the command set causes a watched file to change. In
	/// that case, you should restrict what is watched with --watch and/or --ignore.
	#[clap(
		long,
		help_heading = OPTSET_BEHAVIOUR,
	)]
	pub restart: bool,

	/// Force use of polling for file changes.
	#[clap(
		long,
		help_heading = OPTSET_BEHAVIOUR,
	)]
	pub poll: bool,

	/// Postpone first run until a file changes.
	#[clap(
		long,
		help_heading = OPTSET_BEHAVIOUR,
	)]
	pub postpone: bool,

	/// Sleep some time before running commands.
	///
	/// This adds a delay after a change triggers a run, before actually running the command set.
	/// Equivalent to `-s 'sleep 1'`, except it doesn't spawn a command and is portable.
	#[clap(
		long,
		value_name = "seconds",
		forbid_empty_values = true,
		help_heading = OPTSET_BEHAVIOUR,
	)]
	pub delay_run: Option<u64>,

	/// Quit after a set amount of triggers.
	///
	/// This is mainly useful for testing. Note that it will quit after number "triggers", not
	/// "runs". In cases where a trigger does nothing (doesn't restart the command set), it will
	/// still count down one.
	#[clap(
		long,
		value_name = "number",
		forbid_empty_values = true,
		help_heading = OPTSET_BEHAVIOUR,
	)]
	pub quit_after_n: Option<u8>,

	/// Feature(s) passed to cargo invocations.
	///
	/// This is passed to cargo commands specified with `-x` only, and which start with `b`,
	/// `check`, `doc`, `r`, `test`, or `install`.
	#[clap(
		long = "features",
		help_heading = OPTSET_COMMAND,
	)]
	pub features: Vec<String>,

	/// Cargo command to execute on changes.
	///
	/// The command is lexed using shell logic (allowing for quotes) and executed directly, without a
	/// shell. If you require shell-like features (operators, pipes...), use a --shell command
	/// instead.
	///
	/// If the first word of the command (the cargo subcommand) is `build`, `check`, `doc`,
	/// `install`, `publish`, `run`, `test`, or the standard aliases `b`, `c`, `d`, `r`, `t`, the
	/// features provided with --features, if any, are injected into the command.
	///
	/// If no cargo nor shell commands are provided, `cargo check` is run.
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

	/// Arbitrary (shell) command to execute on changes.
	///
	/// With --use-shell=none, the command is lexed using shell logic (allowing for quotes) and
	/// executed directly, without a shell. Otherwise, it is passed as-is to the shell in use.
	///
	/// See the help for --use-shell for details on which shell is used.
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

	/// File updates debounce delay.
	///
	/// During this time, incoming change events are accumulated and only once the delay has
	/// passed, is an action taken. Note that this does not mean a command will be started: if
	/// --no-restart is given and a command is already running, the outcome of the action will be
	/// to do nothing.
	///
	/// Defaults to 50ms. Parses as decimal seconds by default, but using an integer with the `ms`
	/// suffix may be more convenient. When using --poll mode, you'll want a larger duration, or
	/// risk overloading disk I/O.
	#[clap(
		short = 'd',
		long,
		forbid_empty_values = true,
		help_heading = OPTSET_BEHAVIOUR
	)]
	pub delay: Option<String>,

	/// Ignore a path pattern.
	///
	/// This is in gitignore format. Use a leading `!` for allowlisting.
	///
	/// Unless --no-vcs-ignores is used, `/path` means `{VCS origin}/path` (see --vcs-origin).
	/// If VCS ignores are disabled, `/path` means `{workdir}/path` (see --workdir).
	///
	/// In ignore files, VCS or no, `/path` is bound to the directory the ignore file is in, as per
	/// standard semantics for these files.
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

	/// Load an ignore file.
	///
	/// This will be treated the same as a .ignore file, except that it still works with
	/// --ignore-nothing.
	#[clap(
		long = "ignore-file",
		value_name = "path",
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		help_heading = OPTSET_FILTERING
	)]
	pub ignore_files: Vec<PathBuf>,

	/// Watch a package in a Cargo workspace.
	///
	/// This is almost equivalent to --watch, but with Cargo-aware metadata resolution instead of simple
	/// pathnames.
	///
	/// The `spec` argument uses cargo-pkgid syntax, but implements an extension on that format to
	/// disambiguate in the case of “super workspaces” (see the --super flag). The syntax is also
	/// available in normal crates or workspaces but not very useful.
	///
	/// `bar::foo` matches the `foo` crate under the `bar` workspace. The workspace name is either
	/// the name of the top-level crate (when the [workspace] Cargo.toml also contains a [package]
	/// definition), or the name of the folder containing the [workspace] Cargo.toml (for “virtual”
	/// workspaces). `bar::` matches the `bar` workspace itself.
	///
	/// The argument to --package supports glob patterns, like for cargo-pkgid and other built-in
	/// commands. You'll likely have to quote glob patterns to prevent your shell from eagerly
	/// expanding them.
	///
	/// The syntax does not support the URL version of the pkgid spec format, as that makes no
	/// sense for Cargo Watch's purposes.
	#[clap(
		short = 'p',
		long = "package",
		value_name = "spec pattern",
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		help_heading = OPTSET_FILTERING,
	)]
	pub package_specs: Vec<String>,

	/// Watch the entire Cargo workspace.
	///
	/// When called from the top level of a workspace, this effectively does nothing.
	#[clap(
		long = "workspace",
		help_heading = OPTSET_FILTERING,
	)]
	pub entire_workspace: bool,

	/// Consider super-workspaces when resolving crates.
	///
	/// “Super workspaces” is an xtask pattern where the top level of the project contains only a
	/// .cargo directory containing a config.toml. This setup means that component workspaces are
	/// isolated while sharing an xtask setup, without need for workspace-hack packages.
	///
	/// When this flag is set, Cargo Watch alters its crate resolution mechanisms to look for this
	/// top level super workspace. Nested super workspaces are not supported.
	///
	/// --package resolves specs across all component workspaces.
	///
	/// --workspace watches the entire super workspace.
	///
	/// --workspace-origin's default is set to the super workspace’s root.
	///
	/// The default ignore patterns also change to include the target/ directory under each
	/// component workspace, instead of at the top level.
	///
	/// When run from the root of a super workspace, Cargo Watch will infer the --super flag, and
	/// will watch the entire super workspace unless --package is given.
	#[clap(
		long = "super",
		help_heading = OPTSET_FILTERING,
	)]
	pub super_workspaces: bool,

	/// Watch specific files or folders.
	///
	/// By default, the entire crate is watched. This is resolved from the current directory, and
	/// does not look higher than the first Cargo.toml it finds (except when using --super);
	/// notably this means that when running inside a member crate of a Cargo workspace, only that
	/// particular crate is watched.
	///
	/// Use --workspace to instead watch the entire workspace, and --package to watch a specific
	/// member crate. Use --also-dependencies to watch `path` dependencies of the watched crates as
	/// well, by resolving the Cargo metadata.
	///
	/// When using this option with files, rather than folders, note that in some cases or on some
	/// platforms, events may be missed due to subtleties in how filesystems behave. Prefer to
	/// watch the containing folder instead, with --ignore patterns to limit to the wanted file.
	#[clap(
		short = 'w',
		long,
		value_name = "path",
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		help_heading = OPTSET_FILTERING
	)]
	pub watch: Vec<PathBuf>,

	/// Add watches for local dependencies of watched crates.
	///
	/// This resolves every crate that is watched, either explicitly through --watch and --package,
	/// or implicitly through being a non-ignored descendant of the watches, then resolves all
	/// local (`path = "..."`) dependencies these crates have, and adds the crates to the watched
	/// pathset as if they had been specified with --package.
	///
	/// This process is recursive, so local dependencies of local dependencies will also be
	/// watched, and so on. Note that ignore files can behave oddly in some rare cases, as crates
	/// are added to the watch before ignore files in those crates are discovered.
	#[clap(
		long,
		help_heading = OPTSET_FILTERING,
	)]
	pub also_dependencies: bool,

	/// Override the workspace origin.
	///
	/// The workspace origin is used to resolve Cargo workspace metadata. By default, it is
	/// resolved from the watches or the current working directory.
	#[clap(
		long,
		value_name = "path",
		help_heading = OPTSET_FILTERING,
	)]
	pub workspace_origin: Option<PathBuf>,

	/// Shell to use for --shell commands, or `none` for direct execution.
	///
	/// This applies only to --shell|-s commands; --exec|-x cargo commands are executed directly,
	/// without a shell. The option applies to all *subsequent* shell commands:
	///
	///     $ cargo watch --use-shell=zsh -s one -s two
	///
	/// will use zsh for commands one and two, but:
	///
	///     $ cargo watch -s one --use-shell=zsh -s two
	///
	/// will only use zsh for the second one.
	///
	/// As a convenience, if only one --use-shell is provided and it is used after all command
	/// arguments, it is interpreted as if it was given first:
	///
	///     $ cargo watch -s one -s two --use-shell=zsh
	///
	/// will run both one and two with zsh. (Otherwise the option would do nothing.)
	///
	/// The first word must be the shell program, but it can be followed by options to pass to
	/// the shell program:
	///
	///     $ cargo watch --use-shell='bash -O extglob' -- 'ls !(Cargo.toml)'
	///
	/// On Windows, defaults to Powershell. Elsewhere, defaults to $SHELL, falling back to `sh`
	/// if not available.
	#[clap(
		short = 'S',
		long,
		value_name = "shell",
		multiple_occurrences = true,
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		help_heading = OPTSET_COMMAND,
    )]
	pub use_shell: Vec<String>,

	/// Change the working directory of the commands.
	///
	/// This defaults to the root of the watched crate (see --watch), or to the root of the
	/// workspace if --all is used or multiple member crates are watched.
	///
	/// Note that you cannot change the working directory for subsequent commands by using
	/// `-s 'cd path'`, as each command runs in its own context. Instead use a long shell command
	/// like `-s 'cd path; other commands'`, or run multiple instances of Cargo Watch.
	#[clap(
		short = 'C',
		long,
		value_name = "path",
		help_heading = OPTSET_ENVIRONMENT,
	)]
	pub workdir: Option<PathBuf>,

	/// Send a desktop notification on command start and end.
	///
	/// The message will include success or failure, with the exit code returned by the command.
	#[cfg_attr(target_os = "freebsd", clap(hide = true))]
	#[clap(
		short = 'N',
		long = "notify",
		help_heading = OPTSET_OUTPUT,
	)]
	pub notif: bool,

	/// Inject environment variables into the commands' environments.
	#[clap(
		short = 'E',
		long = "env",
		value_name = "key=value",
		multiple_occurrences = true,
		forbid_empty_values = true,
		min_values = 1,
		number_of_values = 1,
		help_heading = OPTSET_ENVIRONMENT,
    )]
	pub env_vars: Vec<String>,

	/// Inject RUST_BACKTRACE=value into the commands' environments.
	///
	/// Examples: -B=1, -B=full
	#[clap(
		short = 'B',
		value_name = "RUST_BACKTRACE value",
		forbid_empty_values = true,
		help_heading = OPTSET_ENVIRONMENT,
	)]
	pub env_backtrace: Option<String>,

	/// Inject RUST_LOG=value into the commands' environments.
	///
	/// Examples: -L=debug, -L=info,cratename::module=debug
	#[clap(
		short = 'L',
		value_name = "RUST_LOG value",
		forbid_empty_values = true,
		help_heading = OPTSET_ENVIRONMENT,
	)]
	pub env_log: Option<String>,

	/// Don’t inject CARGO_WATCH_* variables in the environment.
	#[clap(
		long = "no-auto-env",
		help_heading = OPTSET_ENVIRONMENT,
	)]
	pub no_auto_env: bool,

	/// Full command to run. -x and -s will be ignored!
	///
	/// The command is lexed using shell logic (allowing for quotes) and executed directly, without a
	/// shell. If you require shell-like features (operators, pipes...), use a --shell command
	/// instead.
	#[clap(
		raw = true,
		value_name = "trailing command",
		help_heading = OPTSET_COMMAND,
	)]
	pub cmd_trail: Vec<String>,
}
