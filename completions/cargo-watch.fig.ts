const completion: Fig.Spec = {
  name: "cargo-watch",
  description: "",
  subcommands: [
    {
      name: "watch",
      description: "Watch your Cargo-based project and run commands when files change",
      options: [
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
          name: "--use-shell",
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
          name: "--testing-only--once",
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
          name: "--no-gitignore",
          description: "Don’t use .gitignore files",
        },
        {
          name: "--no-ignore",
          description: "Don’t use .ignore files",
        },
        {
          name: "--no-restart",
          description: "Don’t restart command while it’s still running",
        },
        {
          name: "--all",
          description: "Reserves for workspace support",
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
          name: "-B",
          description: "Inject RUST_BACKTRACE=value into the command's environment",
        },
        {
          name: "-L",
          description: "Inject RUST_LOG=value into the command's environment",
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
      args: {
        name: "cmd-trail",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      args: {
        name: "subcommand",
        isOptional: true,
      },
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
  ],
};

export default completion;
