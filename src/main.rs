#![feature(macro_rules)]
#![feature(phase)]
#![warn(missing_doc)]
//! Watch files in a Cargo project and compile it when they change

extern crate debug;
extern crate inotify;
#[phase(plugin, link)] extern crate log;

use inotify::wrapper::{INotify, Watch};
use std::io::{Command, fs};
use std::sync::Arc;
use std::sync::atomics::{AtomicBool, SeqCst};

mod cargo;
mod ignore;

macro_rules! Sl(($v:expr) => (String::from_utf8_lossy($v.as_slice())))

fn watch_recursive(ino: &INotify, path: &Path, mask: u32) -> Vec<(Path, Watch)> {
  let mut v: Vec<(Path, Watch)> = Vec::new();

  fn add(ino: &INotify, v: &mut Vec<(Path, Watch)>, path: &Path, mask: u32) {
    match ino.add_watch(path, mask) {
      Ok(w) => v.push((path.clone(), w)),
      Err(e) => error!("Skip path: {} (error: {})", path.display(), e)
    };
  }

  add(ino, &mut v, path, mask.clone());
  match fs::walk_dir(path) {
    Ok(mut i) => {
      for p in i {
        if p.is_dir() {
          add(ino, &mut v, &p, mask.clone());
        }
      }
    },
    Err(e) => error!("Err: {}", e)
  };

  v
}

fn cargo_run(cmd: &str) {
  println!("\n\n$ cargo {}", cmd);
  match Command::new("cargo").arg(cmd).output() {
    Ok(o) => println!("{}\n{}\nExited with: {}", Sl!(o.output), Sl!(o.error), o.status),
    Err(e) => println!("Failed to execute 'cargo {}': {}", cmd, e)
  };
}

fn compile(lock: Arc<AtomicBool>) {
  debug!("Starting a compile");
  cargo_run("build");
  cargo_run("doc");
  lock.store(false, SeqCst);
  debug!("Compile done");
}

fn spawn_compile(lock: &Arc<AtomicBool>) {
  info!("Request to spawn a compile");
  if lock.load(SeqCst) {
    info!("Request denied");
  } else {
    lock.store(true, SeqCst);
    let lock_clone = lock.clone();
    spawn(proc() { compile(lock_clone); });
  }
}

fn main() {
  let mut ino = match INotify::init() {
    Ok(i) => i,
    Err(e) => {
      error!("Failed to init inotify: {}", e);
      std::os::set_exit_status(1);
      return;
    }
  };

  let events = inotify::ffi::IN_CREATE
    | inotify::ffi::IN_DELETE
    | inotify::ffi::IN_MODIFY
    | inotify::ffi::IN_MOVE
    | inotify::ffi::IN_EXCL_UNLINK;

  let compile_lock = Arc::new(AtomicBool::new(false));
  spawn_compile(&compile_lock);
  
  match cargo::root() {
    Some(p) => {
      //let r = watch_recursive(&ino, &p.join("src"), events);
      
      loop {
        match ino.event() {
          Ok(e) => {
            debug!("name: {}", e.name);
            if ignore::filename(&e.name) {
              info!("Ignoring change on '{}'", e.name);
            } else {
              spawn_compile(&compile_lock)
            }
          },
          Err(_) => ()
        }
      }

      //for &(_, w) in r.iter() {
      //  let _ = ino.rm_watch(w);
      //}
      //let _ = ino.close();
    },
    None => {
      error!("Not a Cargo project, aborting.");
      std::os::set_exit_status(64);
    }
  };
}
