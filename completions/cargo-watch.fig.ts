const completion: Fig.Spec = {
  name: "cargo-watch",
  description: "Watch your Cargo-based project and run commands when files change",
  options: [
    {
      name: "--delay-run",
      description: "Sleep some time before running commands",
      args: {
        name: "delay-run",
        isOptional: true,
      },
    },
    {
      name: "--quit-after-n",
      description: "Quit after a set amount of triggers",
      args: {
        name: "quit-after-n",
        isOptional: true,
      },
    },
    {
      name: "--features",
      description: "Feature(s) passed to cargo invocations",
      isRepeatable: true,
      args: {
        name: "features",
        isOptional: true,
      },
    },
    {
      name: ["-x", "--exec"],
      description: "Cargo command(s) to execute on changes",
      isRepeatable: true,
      args: {
        name: "cmd-cargo",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["-s", "--shell"],
      description: "Shell command(s) to execute on changes",
      isRepeatable: true,
      args: {
        name: "cmd-shell",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["-d", "--delay"],
      description: "File updates debounce delay",
      args: {
        name: "delay",
        isOptional: true,
      },
    },
    {
      name: ["-i", "--ignore"],
      description: "Ignore a path pattern",
      isRepeatable: true,
      args: {
        name: "ignores",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["-p", "--package"],
      description: "Reserved for workspace support",
      hidden: true,
      isRepeatable: true,
      args: {
        name: "packages-specs",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["-w", "--watch"],
      description: "Watch specific file(s) or folder(s)",
      isRepeatable: true,
      args: {
        name: "watch",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["-S", "--use-shell"],
      description: "Shell to use for --shell commands, or `none` for direct execution",
      isRepeatable: true,
      args: {
        name: "use-shell",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["-C", "--workdir"],
      description: "Change working directory of the command",
      args: {
        name: "workdir",
        isOptional: true,
      },
    },
    {
      name: ["-E", "--env"],
      description: "Inject environment variables into the commands' environments",
      isRepeatable: true,
      args: {
        name: "env-vars",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: "-B",
      description: "Inject RUST_BACKTRACE=value into the commands' environments",
      args: {
        name: "env-backtrace",
        isOptional: true,
      },
    },
    {
      name: "-L",
      description: "Inject RUST_LOG=value into the commands' environments",
      args: {
        name: "env-log",
        isOptional: true,
      },
    },
    {
      name: ["-h", "--help"],
      description: "Show the help",
    },
    {
      name: ["-V", "--version"],
      description: "Show the version",
    },
    {
      name: ["-c", "--clear"],
      description: "Clear the screen before each run",
    },
    {
      name: "--debug",
      description: "Show debug output",
    },
    {
      name: "--why",
      description: "Show paths that changed",
    },
    {
      name: "--ignore-nothing",
      description: "Ignore nothing, not even target/ and .git/",
    },
    {
      name: "--no-vcs-ignores",
      description: "Don’t use VCS ignore files",
    },
    {
      name: "--no-dot-ignores",
      description: "Don’t use .ignore files",
    },
    {
      name: "--no-restart",
      description: "Don’t restart command while it’s still running",
    },
    {
      name: "--all",
      description: "Reserved for workspace support",
    },
    {
      name: "--poll",
      description: "Force use of polling for file changes",
    },
    {
      name: "--postpone",
      description: "Postpone first run until a file changes",
    },
    {
      name: ["-q", "--quiet"],
      description: "Suppress output from cargo watch itself",
    },
    {
      name: ["-N", "--notify"],
      description: "Send a desktop notification on command start and end",
    },
    {
      name: "--no-auto-env",
      description: "Don’t inject CARGO_WATCH_* variables in the environment",
    },
  ],
  args: {
    name: "cmd-trail",
    isVariadic: true,
    isOptional: true,
  },
};

export default completion;
