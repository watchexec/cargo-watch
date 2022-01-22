use std::{convert::Infallible, time::Duration};

use miette::{Report, Result};
use notify_rust::Notification;
use watchexec::{
	action::{Action, Outcome, PostSpawn, PreSpawn},
	command::Shell,
	config::RuntimeConfig,
	event::ProcessEnd,
	fs::Watcher,
	handler::SyncFnHandler,
	paths::summarise_events_to_env,
	signal::source::MainSignal,
};

use crate::args::Args;

pub fn runtime(args: &Args) -> Result<RuntimeConfig> {
	let mut config = RuntimeConfig::default();

	// TODO: command jiggery
	// config.command(args...);

	config.pathset(&args.watch);

	let delay = (args.delay * 1000.0).round();
	if delay.is_infinite() || delay.is_nan() || delay.is_sign_negative() {
		return Err(Report::msg("delay must be finite and non-negative"));
	}
	if delay >= 1000.0 {
		return Err(Report::msg("delay must be less than 1000 seconds"));
	}

	// SAFETY: delay is finite, not nan, non-negative, and less than 1000
	let delay = Duration::from_millis(unsafe { delay.to_int_unchecked() });
	config.action_throttle(delay);

	if args.poll {
		config.file_watcher(Watcher::Poll(delay));
	}

	// config.command_grouped(args.process_group);

	config.command_shell(if let Some(s) = &args.shell {
		if s.eq_ignore_ascii_case("powershell") {
			Shell::Powershell
		} else if s.eq_ignore_ascii_case("none") {
			Shell::None
		} else if s.eq_ignore_ascii_case("cmd") {
			cmd_shell(s.into())
		} else {
			Shell::Unix(s.into())
		}
	} else {
		default_shell()
	});

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
		if notif {
			Notification::new()
				.summary("Cargo Watch: change detected")
				.body(&format!("Running `{}`", postspawn.command.join(" ")))
				.show()
				.map(drop)
				.unwrap_or_else(|err| {
					eprintln!("[[Failed to send desktop notification: {}]]", err);
				});
		}

		Ok::<(), Infallible>(())
	}));

	Ok(config)
}

#[cfg(windows)]
fn default_shell() -> Shell {
	Shell::Powershell
}

#[cfg(not(windows))]
fn default_shell() -> Shell {
	Shell::default()
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
