#![feature(macro_rules)]
#![feature(phase)]
#![warn(missing_doc)]
//! Watch files in a Cargo project and compile it when they change

extern crate debug;
extern crate inotify;
#[phase(plugin, link)] extern crate log;

use inotify::wrapper::{INotify, Watch};
use std::io::fs;

mod cargo;
mod compile;
mod ignore;
mod timelock;

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

  let t = timelock::new();
  
  match cargo::root() {
    Some(p) => {
      let _ = watch_recursive(&ino, &p.join("src"), events);
      
      loop {
        match ino.event() {
          Ok(e) => compile::handle_event(&t, e),
          Err(_) => ()
        }
      }

      // FIXME: Should really clean these up.
      /* `r` was the result of `watch_recursive()` above
      for &(_, w) in r.iter() {
        let _ = ino.rm_watch(w);
      }
      let _ = ino.close();
      */
    },
    None => {
      error!("Not a Cargo project, aborting.");
      std::os::set_exit_status(64);
    }
  };
}
