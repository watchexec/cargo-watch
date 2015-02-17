#![stable]
//! Utilities for working with cargo,

extern crate libc;

use std::ffi::AsOsStr;
use std::fs::{self, PathExt};
use std::old_io::Command;
use std::old_io::process::StdioContainer;
use std::os;
use std::path::{Path, PathBuf};

macro_rules! Sl(($v:expr) => (String::from_utf8_lossy($v.as_slice())));

/// Returns the closest ancestor Path containing a Cargo.toml.
///
/// Returns None if no ancestor Path contains a Cargo.toml, or if
/// the limit of 10 ancestors has been run through.
#[stable]
pub fn root() -> Option<PathBuf> {
  let mut wd = PathBuf::new(match os::getcwd() {
    Err(_) => { return None; },
    Ok(w) => w
  }.as_os_str());

  if !wd.is_dir() {
    let _ = wd.pop();
  }
  
  fn contains_manifest(path: &mut PathBuf) -> bool {
    match fs::read_dir(path) {
      Ok(mut dirs) => match dirs.find(|p| {
        match *p {
          Err(_) => false,
          Ok(ref d) => { d.path().as_os_str() == "Cargo.toml" }
        }
      }) {
        Some(_) => true,
        None => false
      },
      Err(_) => false
    }
  }

  let mut count = 0u8;
  while !contains_manifest(&mut wd) {
    count += 1;
    if count > 10 || !wd.pop() {
      return None;
    }
  }

  Some(wd)
}

/// Runs a cargo command and displays the output.
#[unstable]
pub fn run(cmd: &str) {
  println!("\n$ cargo {}", cmd);
  match Command::new("cargo")
    .stderr(StdioContainer::InheritFd(libc::STDERR_FILENO))
    .stdout(StdioContainer::InheritFd(libc::STDOUT_FILENO))
    .arg(cmd)
    .output() {
    Ok(o) => println!("-> {}", o.status),
    Err(e) => println!("Failed to execute 'cargo {}': {}", cmd, e)
  };
}
