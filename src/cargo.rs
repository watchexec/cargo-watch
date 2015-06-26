//! Utilities for working with cargo,

use std::ffi::OsStr;
use std::fs::{self, PathExt};
use std::process::Command;
use std::process::Stdio;
use std::env;
use std::path::PathBuf;

macro_rules! Sl(($v:expr) => (String::from_utf8_lossy($v.as_slice())));

/// Returns the closest ancestor Path containing a Cargo.toml.
///
/// Returns None if no ancestor Path contains a Cargo.toml, or if
/// the limit of 10 ancestors has been run through.
pub fn root() -> Option<PathBuf> {
  let mut wd = match env::current_dir() {
    Err(_) => { return None; },
    Ok(w) => w
  };

  fn contains_manifest(path: &mut PathBuf) -> bool {
    match fs::read_dir(path) {
      Ok(mut entries) =>
        entries.any(|ent| match ent {
          Err(_) => false,
          Ok(ref ent) => {
            ent.path().file_name() == Some(OsStr::new("Cargo.toml"))
          }
        }),
      Err(_) => false
    }
  }

  for _ in 0..11 {
    if contains_manifest(&mut wd) {
      return Some(wd)
    }
    if !wd.pop() { break }
  }

  None
}

/// Runs a cargo command and displays the output.
pub fn run(cmd: &str) {
  println!("\n$ cargo {}", cmd);
  match Command::new("cargo")
    .stderr(Stdio::inherit())
    .stdout(Stdio::inherit())
    .arg(cmd)
    .output() {
    Ok(o) => println!("-> {}", o.status),
    Err(e) => println!("Failed to execute 'cargo {}': {}", cmd, e)
  };
}
