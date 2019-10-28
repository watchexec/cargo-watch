//! Utilities for working with cargo and rust files

use std::env;
use std::fs;
use std::path::PathBuf;

/// How many parent folders are searched for a `Cargo.toml`
const MAX_ANCESTORS: u32 = 10;

/// Returns the closest ancestor path containing a `Cargo.toml`.
///
/// Returns `None` if no ancestor path contains a `Cargo.toml`, or if
/// the limit of `MAX_ANCESTORS` ancestors has been reached.
///
/// TODO: #52 Parse toml to get to workspace root
pub fn root() -> Option<PathBuf> {
    /// Checks if the directory contains `Cargo.toml`
    fn contains_manifest(path: &PathBuf) -> bool {
        fs::read_dir(path)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .any(|ent| &ent.file_name() == "Cargo.toml")
            })
            .unwrap_or(false)
    }

    // From the current directory we work our way up, looking for `Cargo.toml`
    env::current_dir().ok().and_then(|mut wd| {
        for _ in 0..MAX_ANCESTORS {
            if contains_manifest(&wd) {
                return Some(wd);
            }
            if !wd.pop() {
                break;
            }
        }

        None
    })
}
