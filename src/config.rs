/// These commands are executed when no arguments are given to `cargo watch`
pub const DEFAULT_COMMANDS: [&'static str; 1] = ["check"];

/// How many parent folders are searched for a `Cargo.toml`
pub const MAX_ANCESTORS: u32 = 10;

/// Which subdirectories are being watched for changes
pub const WATCH_DIRS: [&'static str; 4] = [
    "src",
    "tests",
    "benches",
    "examples",
];

/// Changes on files whose names match one of these regexes are ignored
pub const IGNORED_FILES: [&'static str; 3] = [
    // FIXME: It should be possible to trigger on non-.rs changes (see #31)
    r"[^.][^r][^s]$",
    r"^\.",
    r"^~",
];

/// The timeout for waiting for another process. Shorter means more CPU usage,
/// longer means slower response to file changes. Value in ms.
pub const PROCESS_WAIT_TIMEOUT: u32 = 100;
