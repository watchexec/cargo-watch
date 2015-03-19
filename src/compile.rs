use std::sync::Arc;
use std::sync::atomic::AtomicIsize;
use std::thread;
use super::{Config, cargo, ignore, notify, timelock};

macro_rules! run_if_set(
  ($flag:ident) => {
    if $flag { cargo::run(stringify!($flag)) }
  }
);

fn compile(t: Arc<AtomicIsize>, c: Arc<Config>) {
  let Config {
    build, doc, test, bench
  } = *c;
  debug!("Starting a compile");
  run_if_set!(build);
  run_if_set!(doc);
  run_if_set!(test);
  run_if_set!(bench);
  timelock::update(&t);
  debug!("Compile done");
}

fn spawn_compile(t: &Arc<AtomicIsize>, c: Arc<Config>) {
  info!("Request to spawn a compile");
  // Don't run compiles within less than 2s of each other
  let justnow = timelock::current() - 2;
  let prev = timelock::get(t);
  if prev > justnow {
    info!("Request denied");
  } else {
    timelock::update(t);
    let t2 = t.clone();
    let _ = thread::spawn(move || { compile(t2, c); });
  }
}

pub fn handle_event(t: &Arc<AtomicIsize>, e: notify::Event, c: Arc<Config>) {
  match e.path {
    None => return,
    Some(p) => {
      let name: String = format!("{}", p.display());
      debug!("path: {}", name);
      if ignore::filename(&name) {
        info!("Ignoring change on '{}'", name);
      } else {
        spawn_compile(t, c);
      }
    }
  }
}
