#![unstable]
//! Utilities for working with cargo,

use std::io::{Command, fs};
use std::os;

macro_rules! Sl(($v:expr) => (String::from_utf8_lossy($v.as_slice())))

/// Returns the closest ancestor Path containing a Cargo.toml.
///
/// Returns None if no ancestor Path contains a Cargo.toml, or if
/// the limit of 10 ancestors has been run through.
#[stable]
pub fn root() -> Option<Path> {
  let mut wd = os::getcwd();
  if !wd.is_dir() {
    wd = wd.dir_path();
  }
  
  fn contains_manifest(path: &Path) -> bool {
    match fs::readdir(path) {
      Ok(dirs) => match dirs.iter().find(|path| {
        match path.filename_str() {
          Some(f) => f == "Cargo.toml",
          None => false
        }
      }) {
        Some(_) => true,
        None => false
      },
      Err(_) => false
    }
  }

  let mut count = 0u8;
  while !contains_manifest(&wd) {
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
  println!("\n\n$ cargo {}", cmd);
  match Command::new("cargo").arg(cmd).output() {
    Ok(o) => println!("{}\n{}\nExited with: {}", Sl!(o.output), Sl!(o.error), o.status),
    Err(e) => println!("Failed to execute 'cargo {}': {}", cmd, e)
  };
}
