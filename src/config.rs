use std::path::PathBuf;

/// How many parent folders are searched for a `Cargo.toml`
pub const MAX_ANCESTORS: u32 = 10;

pub fn default_watches() -> Vec<PathBuf> {
    vec![
        "src".into(),
        "tests".into(),
        "benches".into(),
        "examples".into(),
    ]
}

/// Changes on files whose names match one of these regexes are ignored
pub const IGNORED_FILES: [&'static str; 3] = [
    // FIXME: It should be possible to trigger on non-.rs changes (see #31)
    r"[^.][^r][^s]$",
    r"^\.",
    r"^~",
];
