/// These commands are executed when no arguments are given to `cargo watch`
pub const DEFAULT_COMMANDS: [&'static str; 2] = ["build", "test"];

/// How many parent folders are searched for a `Cargo.toml`
pub const MAX_ANCESTORS: u32 = 10;

/// Which subdirectories are being watched for changes
pub const WATCH_DIRS: [&'static str; 3] = ["src", "tests", "benches"];

/// Changes on files whose names match one of these regexes are ignored
pub const IGNORED_FILES: [&'static str; 4] = [
    // FIXME: It should be possible to trigger on non-.rs changes.
    r"[^.][^r][^s]$",
    r"^\.",
    r"~$",
    r"^~",
];
