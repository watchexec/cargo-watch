#![feature(macro_rules)]
#![feature(phase)]
#![warn(missing_docs)]
//! Watch files in a Cargo project and compile it when they change

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
#[phase(plugin)] extern crate docopt_macros;

extern crate notify;
#[phase(plugin, link)] extern crate log;

use notify::{Error, RecommendedWatcher, Watcher};

mod cargo;
mod compile;
mod ignore;
mod timelock;

docopt!(Args deriving Show, "
Usage: cargo-watch [options]
       cargo watch [options]

Options:
  -h, --help      Display this message
  -b, --build     Run `cargo build` when a file is modified
  -d, --doc       Run `cargo doc` when a file is modified
  -t, --test      Run `cargo test` when a file is modified
  -n, --bench     Run `cargo bench` when a file is modified

Default options are `build` and `test`
");

fn main() {
  let (tx, rx) = channel();
  let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);
  let mut watcher = match w {
    Ok(i) => i,
    Err(_) => {
      error!("Failed to init notify");
      std::os::set_exit_status(1);
      return;
    }
  };

  let t = timelock::new();
  match cargo::root() {
    Some(p) => {
      let _ = watcher.watch(&p.join("src"));

      loop {
        match rx.recv_opt() {
          Ok(e) => compile::handle_event(&t, e),
          Err(_) => ()
        }
      }
    },
    None => {
      error!("Not a Cargo project, aborting.");
      std::os::set_exit_status(64);
    }
  }
}
