use std::{convert::Infallible, env, time::Duration};

use miette::{miette, IntoDiagnostic, Report, Result};
use watchexec::{
	action::{Action, Outcome, PostSpawn, PreSpawn},
	command::{Command, Shell},
	config::RuntimeConfig,
	event::ProcessEnd,
	fs::Watcher,
	handler::SyncFnHandler,
	paths::summarise_events_to_env,
	signal::source::MainSignal,
};

#[cfg(not(target_os = "freebsd"))]
use notify_rust::Notification;

use crate::args::Args;

pub fn runtime(args: &Args, command_order: Vec<&'static str>) -> Result<RuntimeConfig> {
	let mut config = RuntimeConfig::default();

	let features = if args.features.is_empty() {
		None
	} else {
		Some(args.features.join(","))
	};

	let mut used_shell = if args.use_shell.len() == 1 && command_order.last() == Some(&"use-shell")
	{
		args.use_shell.first().cloned()
	} else {
		None
	};

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
					&used_shell,
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

	config.pathset(&args.watch);

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

	// config.command_grouped(args.process_group);

	let clear = args.clear;
	let notif = args.notif;
	let on_busy = if args.no_restart {
		"do-nothing"
	} else {
		"restart"
	};

	// TODO: add using SubSignal in Args directly
	// let mut signal = args
	// 	.signal
	// 	.map(SubSignal::from_str)
	// 	.transpose()
	// 	.into_diagnostic()?
	// 	.unwrap_or(SubSignal::Terminate);

	let print_events = args.why;
	let once = args.once;

	config.on_action(move |action: Action| {
		let fut = async { Ok::<(), Infallible>(()) };

		if print_events {
			for (n, event) in action.events.iter().enumerate() {
				eprintln!("[EVENT {}] {}", n, event);
			}
		}

		if once {
			action.outcome(Outcome::both(Outcome::Start, Outcome::wait(Outcome::Exit)));
			return fut;
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
					Some(ProcessEnd::Success) => ("Command was successful".to_string(), false),
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

				action.outcome(Outcome::DoNothing);
				return fut;
			}
		}

		let when_running = match (clear, on_busy) {
			(_, "do-nothing") => Outcome::DoNothing,
			(true, "restart") => {
				Outcome::both(Outcome::Stop, Outcome::both(Outcome::Clear, Outcome::Start))
			}
			(false, "restart") => Outcome::both(Outcome::Stop, Outcome::Start),
			// (_, "signal") => Outcome::Signal(signal),
			// (true, "queue") => Outcome::wait(Outcome::both(Outcome::Clear, Outcome::Start)),
			// (false, "queue") => Outcome::wait(Outcome::Start),
			_ => Outcome::DoNothing,
		};

		let when_idle = if clear {
			Outcome::both(Outcome::Clear, Outcome::Start)
		} else {
			Outcome::Start
		};

		action.outcome(Outcome::if_running(when_running, when_idle));

		fut
	});

	let no_env = false; // args.is_present("no-environment");
	config.on_pre_spawn(move |prespawn: PreSpawn| async move {
		if !no_env {
			let envs = summarise_events_to_env(prespawn.events.iter());
			if let Some(mut command) = prespawn.command().await {
				for (k, v) in envs {
					command.env(format!("CARGO_WATCH_{}_PATH", k), v);
				}
			}
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

fn cargo_command(arg: &String, features: &Option<String>) -> Result<Command> {
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

fn shell_command(arg: &String, use_shell: &Option<String>) -> Result<Command> {
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
		command: arg.clone(),
	})
}
