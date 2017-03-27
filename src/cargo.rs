//! Utilities for working with cargo and rust files

use config;
use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Returns the closest ancestor path containing a `Cargo.toml`.
///
/// Returns `None` if no ancestor path contains a `Cargo.toml`, or if
/// the limit of MAX_ANCESTORS ancestors has been reached.
///
/// TODO: #52 Parse toml to get to workspace root
pub fn root() -> Option<PathBuf> {
    /// Checks if the directory contains `Cargo.toml`
    fn contains_manifest(path: &PathBuf) -> bool {
        fs::read_dir(path).map(|entries| {
            entries.filter_map(|res| res.ok())
                   .any(|ent| &ent.file_name() == "Cargo.toml")
        }).unwrap_or(false)
    }

    // From the current directory we work our way up, looking for `Cargo.toml`
    env::current_dir().ok().and_then(|mut wd| {
        for _ in 0..config::MAX_ANCESTORS {
            if contains_manifest(&mut wd) {
                return Some(wd);
            }
            if !wd.pop() {
                break;
            }
        }

        None
    })
}

lazy_static! {
    static ref IGNORED_FILES: Vec<Regex> = {
        config::IGNORED_FILES.iter().map(|s| {
            // FIXME: This should use the compile-time `regex!` macros, when
            // syntax extensions become stabilized (see #32)
            Regex::new(s).expect("Couldn't parse regex")
        }).collect()
    };
}

/// Checks if the given filename should be ignored
pub fn is_ignored_file(f: &str) -> bool {
    IGNORED_FILES.iter().any(|fr| fr.is_match(f))
}
