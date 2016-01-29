//! Utilities for working with cargo,

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

macro_rules! Sl(($v:expr) => (String::from_utf8_lossy($v.as_slice())));

/// Returns the closest ancestor path containing a `Cargo.toml`.
///
/// Returns `None` if no ancestor path contains a `Cargo.toml`, or if
/// the limit of 10 ancestors has been reached.
pub fn root() -> Option<PathBuf> {
    /// Checks if the directory contains `Cargo.toml`
    fn contains_manifest(path: &PathBuf) -> bool {
        fs::read_dir(path).map(|entries| {
            entries.filter_map(|res| res.ok())
                   .any(|ent| &ent.file_name() == "Cargo.toml")
        }).unwrap_or(false)
    }

    let mut wd = match env::current_dir() {
        Err(_) => {
            return None;
        }
        Ok(w) => w,
    };


    // TODO: put constant somewhere else
    for _ in 0..11 {
        if contains_manifest(&mut wd) {
            return Some(wd);
        }
        if !wd.pop() {
            break;
        }
    }

    None
}

/// Runs one or more cargo commands and displays the output.
pub fn run(cmds: &str) {
    let cmds_vec: Vec<&str> = cmds.split_whitespace().collect();
    println!("\n$ cargo {}", cmds);
    match Command::new("cargo")
              .args(&cmds_vec)
              .status() {
        Ok(status) => println!("-> {}", status),
        Err(e) => println!("Failed to execute 'cargo {}': {}", cmds, e),
    };
}
