use std::sync::Arc;
use std::sync::atomic::AtomicIsize;
use std::thread;
use super::{Config, cargo, ignore, notify, timelock};

fn compile(t: Arc<AtomicIsize>, c: Arc<Config>) {
  let Config {
    ref args
  } = *c;
  debug!("Starting a compile");
  if args.len() > 0 {
      args.iter().map(|v| cargo::run(v)).last();
  }
  else {
      vec![String::from("build"), String::from("test")].iter().map(|v| cargo::run(v)).last();
  }
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
      debug!("path: {}", p.display());
      match p.file_name() {
        None => return,
        Some(f) => {
            let name = f.to_string_lossy();
            if ignore::filename(&name) {
                info!("Ignoring change on '{}' ({})", name, p.display());
            } else {
                spawn_compile(t, c);
            }
        }
      }
    }
  }
}
