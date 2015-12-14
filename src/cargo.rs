//! Utilities for working with cargo,

#[cfg(all(feature = "notifications", target_os="linux"))]
extern crate notify_rust;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::process::Output;

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

/// Runs one or more cargo commands and displays the output.
pub fn run(cmds: &str) {
  let cmds_vec: Vec<&str> = cmds.split_whitespace().collect();
  println!("\n$ cargo {}", cmds);
  match Command::new("cargo")
    .stderr(Stdio::inherit())
    .stdout(Stdio::inherit())
    .args(&cmds_vec)
    .output() {
    Ok(o)  => notify(&o, &cmds),
    Err(e) => println!("Failed to execute 'cargo {}': {}", cmds, e)
  };
}

#[cfg(not(feature = "notifications"))]
fn notify(output:&Output, cmds:&str){
    if output.status.success() {
        println!("{} successfull", cmds );
    } else {
        println!("{} not successfull", cmds );
    }
}


/// Sends notification after command has been completed.
#[cfg(all(feature = "notifications", target_os="linux"))]
fn notify(output:&Output, cmds:&str){

    if output.status.success() {
        println!("{} successfull", cmds );

        notify_rust::Notification::new()
            .summary(&format!("Cargo Watch Ok: \"{}\"", cmds))
            //.body("crate would probably compile")
            .icon("dialog-ok")
            .show().unwrap();
    } else {
        println!("{} not successfull", cmds );
        notify_rust::Notification::new()
            .summary(&format!("Cargo Watch Failed: \"{}\"", cmds))
            //.body("crate would not compile")
            .icon("dialog-cancel")
            .show().unwrap();
    }

}

