use std::sync::Arc;
use std::sync::atomic::AtomicInt;
use super::{cargo, ignore, notify, timelock};

fn compile(t: Arc<AtomicInt>) {
  debug!("Starting a compile");
  cargo::run("build");
  cargo::run("doc");
  timelock::update(&t);
  debug!("Compile done");
}

fn spawn_compile(t: &Arc<AtomicInt>) {
  info!("Request to spawn a compile");
  // Don't run compiles within less than 2s of each other
  let justnow = timelock::current() - 2;
  let prev = timelock::get(t);
  if prev > justnow {
    info!("Request denied");
  } else {
    timelock::update(t);
    let t2 = t.clone();
    spawn(move || { compile(t2); });
  }
}

pub fn handle_event(t: &Arc<AtomicInt>, e: notify::Event) {
  match e.path {
    None => return,
    Some(p) => {
      let name: String = format!("{}", p.display());
      debug!("path: {}", name);
      if ignore::filename(&name) {
        info!("Ignoring change on '{}'", name);
      } else {
        spawn_compile(t);
      }
    }
  }
}
