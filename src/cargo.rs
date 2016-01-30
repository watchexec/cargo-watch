//! Utilities for working with cargo and rust files

use std::env;
use std::fs;
use std::path::PathBuf;
use regex::Regex;

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

    // From the current directory we work our way up, looking for `Cargo.toml`
    env::current_dir().ok().and_then(|mut wd| {
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
    })
}

// FIXME: This should use the compile-time `regex!` macros, when syntax
// extensions become stabilized.
macro_rules! unwrap_regex {
    ($r:expr) => {
        Regex::new($r).expect("Couldn't parse regex")
    }
}

lazy_static! {
    static ref IGNORED_FILES: Vec<Regex> = {
        // FIXME: It should be possible to trigger on non-.rs changes.
        // Currently the first regex prevents that.
        vec![
            unwrap_regex!(r"[^.][^r][^s]$"),
            unwrap_regex!(r"^\."),
            unwrap_regex!(r"~$"),
            unwrap_regex!(r"^~"),
        ]
    };
}


pub fn is_ignored_file(f: &str) -> bool {
    IGNORED_FILES.iter().any(|fr| fr.is_match(f))
}
