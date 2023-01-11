use std::{
	convert::Infallible,
	env,
	path::PathBuf,
	sync::{
		atomic::{AtomicU8, Ordering},
		Arc,
	},
	time::Duration,
};

use miette::{miette, IntoDiagnostic, Report, Result};
use tracing::{debug, info};
use watchexec::{
	action::{Action, Outcome, PostSpawn, PreSpawn},
	command::{Command, Shell},
	config::RuntimeConfig,
	event::{ProcessEnd, Tag},
	fs::Watcher,
	handler::SyncFnHandler,
	keyboard::Keyboard,
	paths::summarise_events_to_env,
	signal::source::MainSignal,
};

#[cfg(not(target_os = "freebsd"))]
use notify_rust::Notification;

use crate::args::Args;

pub fn runtime(args: &Args, command_order: Vec<&'static str>) -> Result<RuntimeConfig> {
	let mut config = RuntimeConfig::default();

	let mut pathset = args.watch.clone();
	if pathset.is_empty() {
		pathset = vec![PathBuf::from(".")];
	}
	config.pathset(&pathset);

	let features = if args.features.is_empty() {
		None
	} else {
		Some(args.features.join(","))
	};
	info!(?features, "features");

	let mut used_shell = if args.use_shell.len() == 1 && command_order.last() == Some(&"use-shell")
	{
		args.use_shell.first().cloned()
	} else {
		None
	};
	debug!(?used_shell, "initial used shell");

	if !args.cmd_trail.is_empty() {
		info!(trailing=?args.cmd_trail, "use the trailing command");
		let mut args = args.cmd_trail.clone();
		config.command(Command::Exec {
			prog: args.remove(0),
			args,
		});
	} else if args.cmd_cargo.is_empty() && args.cmd_shell.is_empty() {
		info!("use the default command");
		config.command(cargo_command("check", &features)?);
	} else {
		info!("use the optioned commands");
		let mut commands = Vec::with_capacity(args.cmd_cargo.len() + args.cmd_shell.len());

		let mut cargos = args.cmd_cargo.iter();
		let mut shells = args.cmd_shell.iter();
		let mut use_shells = args.use_shell.iter();

		for c in command_order {
			match c {
				"cargo" => {
					commands.push(cargo_command(
						cargos
							.next()
							.ok_or_else(|| miette!("Argument-order mismatch, this is a bug"))?,
						&features,
					)?);
				}
				"shell" => {
					commands.push(shell_command(
						shells
							.next()
							.ok_or_else(|| miette!("Argument-order mismatch, this is a bug"))?,
						used_shell.as_ref(),
					)?);
				}
				"use-shell" => {
					used_shell.replace(
						use_shells
							.next()
							.ok_or_else(|| miette!("Argument-order mismatch, this is a bug"))?
							.clone(),
					);
				}
				_ => {}
			}
		}

		config.commands(commands);
	}

	if let Some(delay) = &args.delay {
		let delay = if delay.ends_with("ms") {
			let d: u64 = delay.trim_end_matches("ms").parse().into_diagnostic()?;
			Duration::from_millis(d)
		} else {
			let d: f64 = delay.parse().into_diagnostic()?;
			let delay = (d * 1000.0).round();
			if delay.is_infinite() || delay.is_nan() || delay.is_sign_negative() {
				return Err(Report::msg("delay must be finite and non-negative"));
			}
			if delay >= 1000.0 {
				return Err(Report::msg("delay must be less than 1000 seconds"));
			}

			// SAFETY: delay is finite, not nan, non-negative, and less than 1000
			Duration::from_millis(unsafe { delay.to_int_unchecked() })
		};

		config.action_throttle(delay);
	}

	if args.poll {
		config.file_watcher(Watcher::Poll(config.action.throttle));
	}

	config.keyboard_emit_eof(args.stdin_quit);

	// config.command_grouped(args.process_group);

	let quiet = args.quiet;
	let clear = args.clear;
	let notif = args.notif;
	let on_busy = if args.restart {
		"restart"
	} else {
		"do-nothing"
	};

	// TODO: add using SubSignal in Args directly
	// let mut signal = args
	// 	.signal
	// 	.map(SubSignal::from_str)
	// 	.transpose()
	// 	.into_diagnostic()?
	// 	.unwrap_or(SubSignal::Terminate);

	let print_events = args.why;
	let quit_after_n = args.quit_after_n.map(|n| Arc::new(AtomicU8::new(n)));
	let delay_run = args.delay_run.map(Duration::from_secs);

	config.on_action(move |action: Action| {
		let fut = async { Ok::<(), Infallible>(()) };

		if print_events && !clear {
			for (n, event) in action.events.iter().enumerate() {
				eprintln!("[EVENT {}] {}", n, event);
			}
		}

		let signals: Vec<MainSignal> = action.events.iter().flat_map(|e| e.signals()).collect();
		let has_paths = action
			.events
			.iter()
			.flat_map(|e| e.paths())
			.next()
			.is_some();

		if signals.contains(&MainSignal::Terminate) {
			action.outcome(Outcome::both(Outcome::Stop, Outcome::Exit));
			return fut;
		}

		if signals.contains(&MainSignal::Interrupt) {
			action.outcome(Outcome::both(Outcome::Stop, Outcome::Exit));
			return fut;
		}

		let is_keyboard_eof = action
			.events
			.iter()
			.any(|e| e.tags.contains(&Tag::Keyboard(Keyboard::Eof)));

		if is_keyboard_eof {
			action.outcome(Outcome::both(Outcome::Stop, Outcome::Exit));
			return fut;
		}

		if !has_paths {
			if !signals.is_empty() {
				let mut out = Outcome::DoNothing;
				for sig in signals {
					out = Outcome::both(out, Outcome::Signal(sig.into()));
				}

				action.outcome(out);
				return fut;
			}

			let completion = action.events.iter().flat_map(|e| e.completions()).next();
			if let Some(status) = completion {
				let (msg, printit) = match status {
					Some(ProcessEnd::ExitError(code)) => {
						(format!("Command exited with {}", code), true)
					}
					Some(ProcessEnd::ExitSignal(sig)) => {
						(format!("Command killed by {:?}", sig), true)
					}
					Some(ProcessEnd::ExitStop(sig)) => {
						(format!("Command stopped by {:?}", sig), true)
					}
					Some(ProcessEnd::Continued) => ("Command continued".to_string(), true),
					Some(ProcessEnd::Exception(ex)) => {
						(format!("Command ended by exception {:#x}", ex), true)
					}
					Some(ProcessEnd::Success) => ("Command was successful".to_string(), !quiet),
					None => ("Command completed".to_string(), false),
				};

				if printit {
					eprintln!("[[{}]]", msg);
				}

				#[cfg(not(target_os = "freebsd"))]
				if notif {
					Notification::new()
						.summary("Watchexec: command ended")
						.body(&msg)
						.show()
						.map(drop)
						.unwrap_or_else(|err| {
							eprintln!("[[Failed to send desktop notification: {}]]", err);
						});
				}

				if let Some(runs) = quit_after_n.clone() {
					if runs.load(Ordering::SeqCst) == 0 {
						eprintln!("[[--quit after n--]]");
						action.outcome(Outcome::Exit);
						return fut;
					}
				}

				action.outcome(Outcome::DoNothing);
				return fut;
			}
		}

		if let Some(runs) = quit_after_n.clone() {
			if runs.load(Ordering::SeqCst) == 0 {
				debug!("quitting after n triggers");
				action.outcome(Outcome::Exit);
				return fut;
			}
		}

		let start = if clear {
			Outcome::both(Outcome::Clear, Outcome::Start)
		} else {
			Outcome::Start
		};

		let start = if let Some(delay) = &delay_run {
			Outcome::both(Outcome::Sleep(*delay), start)
		} else {
			start
		};

		let when_idle = start.clone();
		let when_running = match on_busy {
			"do-nothing" => Outcome::DoNothing,
			"restart" => Outcome::both(Outcome::Stop, start),
			// "signal" => Outcome::Signal(signal),
			_ => Outcome::DoNothing,
		};

		if let Some(runs) = quit_after_n.clone() {
			let remaining = runs.fetch_sub(1, Ordering::SeqCst);
			if remaining > 0 {
				debug!(?remaining, "getting closer to quitting");
			}
		}

		action.outcome(Outcome::if_running(when_running, when_idle));

		fut
	});

	let no_env = args.no_auto_env;
	config.on_pre_spawn(move |prespawn: PreSpawn| async move {
		if !no_env {
			let envs = summarise_events_to_env(prespawn.events.iter());
			if let Some(mut command) = prespawn.command().await {
				for (k, v) in envs {
					command.env(format!("CARGO_WATCH_{}_PATH", k), v);
				}
			}
		}

		// with --clear, printing the events right before clearing is useless.
		// this way is a bit incorrect as we'll print before every command, but
		// it will be actually useful. Perhaps this warrants a new Watchexec hook
		// for post- or on- outcomes...
		if print_events && clear {
			for (n, event) in prespawn.events.iter().enumerate() {
				eprintln!("[EVENT {}] {}", n, event);
			}
		}

		if !quiet {
			eprintln!("[[Running `{}`]]", prespawn.command);
		}

		Ok::<(), Infallible>(())
	});

	config.on_post_spawn(SyncFnHandler::from(move |postspawn: PostSpawn| {
		#[cfg(not(target_os = "freebsd"))]
		if notif {
			Notification::new()
				.summary("Cargo Watch: change detected")
				.body(&format!("Running `{}`", postspawn.command))
				.show()?;
		}

		#[cfg(target_os = "freebsd")]
		return Ok::<(), Infallible>(());
		#[cfg(not(target_os = "freebsd"))]
		return Ok::<(), notify_rust::error::Error>(());
	}));

	Ok(config)
}

#[cfg(windows)]
fn default_shell() -> Shell {
	Shell::Powershell
}

#[cfg(not(windows))]
fn default_shell() -> Shell {
	Shell::Unix(env::var("SHELL").unwrap_or_else(|_| String::from("sh")))
}

// because Shell::Cmd is only on windows
#[cfg(windows)]
fn cmd_shell(_: String) -> Shell {
	Shell::Cmd
}

#[cfg(not(windows))]
fn cmd_shell(s: String) -> Shell {
	Shell::Unix(s)
}

fn cargo_command(arg: &str, features: &Option<String>) -> Result<Command> {
	debug!(command=?arg, ?features, "building a cargo command");

	let mut lexed = shlex::split(arg).ok_or_else(|| miette!("Command is not valid: {:?}", arg))?;
	let subc = lexed
		.get(0)
		.ok_or_else(|| miette!("Cargo command needs at least one word"))?
		.clone();

	if let Some(features) = features.as_ref() {
		if subc.starts_with('b')
			|| subc == "check"
			|| subc == "doc"
			|| subc.starts_with('r')
			|| subc == "test"
			|| subc == "install"
		{
			lexed.insert(1, "--features".into());
			lexed.insert(2, features.into());
		}
	}

	Ok(Command::Exec {
		prog: "cargo".into(),
		args: lexed,
	})
}

fn shell_command(arg: &str, use_shell: Option<&String>) -> Result<Command> {
	debug!(command=?arg, ?use_shell, "building a shelled command");

	let (shell, shell_args) = if let Some(sh) = use_shell {
		let mut lexed_shell = shlex::split(&sh)
			.ok_or_else(|| miette!("Shell invocation syntax is invalid: {:?}", sh))?;
		let shell_prog = lexed_shell.remove(0);

		(
			if shell_prog.eq_ignore_ascii_case("powershell") {
				Shell::Powershell
			} else if shell_prog.eq_ignore_ascii_case("none") {
				// for now, silently discard any shell arguments provided
				let mut lexed =
					shlex::split(arg).ok_or_else(|| miette!("Command is not valid: {:?}", arg))?;
				let prog = lexed.remove(0);
				return Ok(Command::Exec { prog, args: lexed });
			} else if shell_prog.eq_ignore_ascii_case("cmd") {
				cmd_shell(shell_prog.into())
			} else {
				Shell::Unix(shell_prog.into())
			},
			lexed_shell,
		)
	} else {
		(default_shell(), Vec::new())
	};

	Ok(Command::Shell {
		shell,
		args: shell_args,
		command: arg.into(),
	})
}
