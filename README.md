# $ cargo watch

[![Crate release version](https://flat.badgen.net/crates/v/cargo-watch)](https://crates.io/crates/cargo-watch)
[![Crate license: CC0 1.0](https://flat.badgen.net/github/license/watchexec/cargo-watch)](https://creativecommons.org/publicdomain/zero/1.0/)
[![Crate download count](https://flat.badgen.net/crates/d/cargo-watch)](https://crates.io/crates/cargo-watch)
[![CI status](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml/badge.svg)](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml)

Cargo Watch watches over your project's source for changes, and runs Cargo
commands when they occur.

If you've used [nodemon], [guard], or [entr], it will probably feel familiar.

**I recommend [Bacon] or [Watchexec] instead.** (see below for more)

[nodemon]: http://nodemon.io/
[entr]: https://github.com/eradman/entr
[guard]: http://guardgem.org/
[Bacon]: https://dystroy.org/bacon/

- In the public domain / licensed with CC0.

## Maintenance

Cargo Watch is on life support: it will not receive further updates, but does remain available.

I (@passcod) currently have very little time to dedicate to unpaid OSS.
There is a significant amount of work I deem required to get Watchexec (the library) to a good-enough state to bring its improvements to Cargo Watch, and that has been the case for years without a realistic end in sight.
I have dwindling motivation in the face of having spent 10 years on or around this project and its dependencies (it was a long while ago, but once upon a time the Notify library was spun off from Cargo Watch!), when at the very start, this tool was only made to clear a quick hurdle that I'd encountered while trying to code _other, probably more interesting, yet now long-forgotten_ Rust adventures.

However, not all is lost, dear users.
For almost the entire life of the project, I have had a thought: that someone with more resources, skill, time, and/or the benefit of hindsight would come around and make something _better_.
Granted, I thought this would happen to Notify.
But Notify has persisted, has been passed on to live a long life, and instead the contender is [Bacon].

I have had no involvement in Bacon.
Yet it is everything I have wanted to achieve in Cargo Watch.
Indeed some five years ago I started development on a Cargo Watch replacement I called "Overwatch", which would have a TUI, a tasks file, a rich pager, and more long-desired features.
That never eventuated, though a lot of the low-level improvements that I wrote in preparation for Overwatch "made it" into Notify version 5 and the Watchexec library version 2.
Bacon today is what I wanted Overwatch to be.

Let's face it: Cargo Watch has gone through too many incremental changes, with too little overarching design.
It sports no less than four different syntaxes to run commands.
Its lackluster filtering options can be obnoxious to use.
Pager support is non-existent, sometimes requiring arcane invocations to get right.
It can conflict with Rust Analyzer (which didn't exist 10 years ago!), though that has improved a lot over the years.

It's time to let it go.
Use [Bacon].
Remember Cargo Watch.

## Install

<a href="https://repology.org/project/cargo-watch/versions"><img align="right" src="https://repology.org/badge/vertical-allrepos/cargo-watch.svg" alt="Packaging status"></a>

With [cargo-binstall](https://github.com/ryankurte/cargo-binstall):

```console
$ cargo install cargo-watch
```

From source:

```console
$ cargo install cargo-watch --locked
```

Or clone and build with `$ cargo build` then place in your $PATH.

You can also install from the pre-built binaries available **[on the release page][releases]**.

[releases]: https://github.com/watchexec/cargo-watch/releases/latest

### Auxiliary

This repository contains a [manual page](./cargo-watch.1) and
[Zsh completions](./completions) that you may want to install.

## Usage

By default, it runs `check`. You can easily override this, though:

```
$ cargo watch [-x command]...
```

A few examples:

```
# Run tests only
$ cargo watch -x test

# Run check then tests
$ cargo watch -x check -x test

# Run run with arguments
$ cargo watch -x 'run -- --some-arg'

# Run an arbitrary command
$ cargo watch -- echo Hello world

# Run with features passed to cargo
$ cargo watch --features "foo,bar"
```

There's a lot more you can do! Here's a copy of the help:

```
USAGE:
    cargo watch [FLAGS] [OPTIONS]

FLAGS:
    -c, --clear              Clear the screen before each run
    -h, --help               Display this message
        --ignore-nothing     Ignore nothing, not even target/ and .git/
        --debug              Show debug output
        --why                Show paths that changed
    -q, --quiet              Suppress output from cargo-watch itself
        --no-vcs-ignores       Don’t use .gitignore files
        --no-dot-ignores          Don’t use .ignore files
        --no-restart         Don’t restart command while it’s still running
    -N, --notify             Send a desktop notification when watchexec notices a change
                             (experimental, behaviour may change)
        --poll               Force use of polling for file changes
        --postpone           Postpone first run until a file changes
        --skip-local-deps    Don't try to find local dependencies of the current crate and watch
                             their working directories. Only watch the current directory.
    -V, --version            Display version information
        --watch-when-idle    Ignore events emitted while the commands run.

OPTIONS:
    -x, --exec <cmd>...            Cargo command(s) to execute on changes [default: check]
    -s, --shell <cmd>...           Shell command(s) to execute on changes
    -d, --delay <delay>            File updates debounce delay in seconds [default: 0.5]
        --features <features>      List of features passed to cargo invocations
    -i, --ignore <pattern>...      Ignore a glob/gitignore-style pattern
    -B <rust-backtrace>            Inject RUST_BACKTRACE=VALUE (generally you want to set it to 1)
                                   into the environment
        --use-shell <use-shell>    Use a different shell. E.g. --use-shell=bash
    -w, --watch <watch>...         Watch specific file(s) or folder(s). Disables finding and
                                   watching local dependencies.
    -C, --workdir <workdir>        Change working directory before running command [default: crate
                                   root]

ARGS:
    <cmd:trail>...    Full command to run. -x and -s will be ignored!

Cargo commands (-x) are always executed before shell commands (-s). You can use the `-- command`
style instead, note you'll need to use full commands, it won't prefix `cargo` for you.

By default, the workspace directories of your project and all local dependencies are watched,
except for the target/ and .git/ folders. Your .ignore and .gitignore files are used to filter
paths.

On Windows, patterns given to -i have forward slashes (/) automatically
converted to backward ones (\) to ease command portability.
```

### Ignore files

`.gitignore` files are used by default to ignore paths to watch and trigger
runs. To stop honouring them, pass `--no-vcs-ignores`.

`.ignore` files in the same syntax are also used by default. This file can be
used to specify files that should be ignored by cargo watch but checked into
git, without constantly adding `--ignore abc` options on the command-line. Do
note that `.ignore` files may also be used by other programs, like
[ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md#automatic-filtering).
To stop honouring these, pass `--no-dot-ignores`.

Cargo watch also has an internal list of default ignores on top of those
specified in files, like `target/` and `.git/` and various other common types
(logs, editor swap files, lockfiles, etc).

To skip absolutely all ignores, use the `--ignore-nothing` flag.

### Ignore syntax

See the [Glob patterns page][globdoc] for a description of how they work in the
context of this tool. That’s the syntax used for the `--ignore` option.

Additionally, some specific quirks and behaviours:

- On Windows, patterns should be specified with Windows-style (`\\`) separators.
Unix-style separators (`/`) would not match Windows paths, which could be
confusing and give the appearance of commandline ignores not working.

- From Cargo Watch 7.0.0, `/` in commandline ignores are automatically translated
to `\\` when running on Windows, but one should still try to write the correct
patterns for the platform, as there may be more subtle differences.

- From Cargo Watch 7.3.0, `--ignore` patterns were fixed to provide better
experience with directory matching. Previously, ignoring a folder would need
unyieldy `-i folder/**` patterns; now that is handled internally, and only `-i
folder` is needed for the same effect.

[globdoc]: https://watchexec.github.io/docs/glob-patterns.html

### Reloading servers seamlessly

Cargo Watch pairs very well with [systemfd]/[Catflap], tools for Unixy platforms that
lets one spawn a socket before the watcher runs that Rust servers can then bind
to, avoiding request-dropping and the infamous ADDRINUSE error. For example:

```
$ systemfd --no-pid -s http::5000 -- cargo watch -x run
```

[Catflap]: https://github.com/watchexec/catflap
[systemfd]: https://github.com/mitsuhiko/systemfd

Of course, if you don't need to guard against these issues or don't want to
modify your program to grab sockets instead of ports, you can use Cargo Watch
as-is: it will happily just restart your server normally.

### Restarting an application only if the build/check succeeds

Supervising and starting/restarting/stopping long-running processes is explicitly not within Cargo Watch's remit.
Instead, you should use a process manager.
On most Linuxes, `systemd-run --user` is greatly useful here.
On other platforms, a tool such as [pm2] or [pmc] can be used.

[pm2]: https://pm2.keymetrics.io
[pmc]: https://lib.rs/crates/pmc

#### With systemd-run

Start the application service:

```
$ systemd-run --user --pty --unit myappserver cargo run
```

Restart after a successful compile:

```
$ cargo -x check -x build -s 'systemctl --user restart myappserver'
```

#### With [pm2]

Start the application service:

```
$ pm2 start --name myappserver cargo run
$ pm2 logs -f myappserver
```

Restart after a successful compile:

```
$ cargo -x check -x build -s 'pm2 restart myappserver'
```

#### Or with cargo watch alone

This uses a "trigger file" that's watched by a second cargo watch to manage a process.

Start the application service:

```
$ touch .trigger
$ cargo watch --no-vcs-ignores -w .trigger -x run
```

Restart after a successful compile:

```
$ cargo watch -x check -x build -s 'touch .trigger'
```

The `--no-vcs-ignores` flag ensures that you can safely add `.trigger` to your
`.gitignore` file to avoid mistakenly committing it.

## Troubleshooting

In all cases, start by checking your version with `cargo watch --version` and,
if necessary, upgrading to [the latest one][releases].

### RLS is slow while using cargo watch, or vice versa, or it's waiting for the project lock a lot

Cargo builds (and checks, and clippy, and tests because the tests have to be
built) take out a lock on the project so two cargo instances don't run at the
same time.

However, Rust Analyzer is much better at this, so use that instead of RLS.

### On Windows 7 (or lower): "failed to add to job object: Access denied (OS Error 5)"

Cargo Watch versions 5.0.0 and up (and Watchexec versions 1.3.0 and up) **[do
not support Windows 7 or lower][i-69].** Support _will not_ be added. Issues for
Windows <=7 will be closed. If it works, lucky you, but that is not intentional.

[i-69]: https://github.com/watchexec/cargo-watch/issues/69

### I want to run cargo-watch directly, without going through cargo

You can! But you'll have to specify the `watch` subcommand as the first
argument, like so:

```
$ /path/to/cargo-watch watch -x build
```

### I want to run cargo-watch outside of a Cargo project

That's not supported. If you have a good reason to use a Cargo-specific tool
outside a Cargo project, please open an issue! Otherwise, you'll probably be
best served with using [Watchexec].

### If file updates seems to never trigger

Try using `--poll` to force the polling fallback.

If that still doesn't work, and you're using an editor that does "safe saving",
like IntelliJ / PyCharm, you may have to disable "safe saving" as that may
prevent file notifications from being generated properly.

Also try using the `--why` option to see if the paths you expect are changing.

### Linux: If it fails to watch some deep directories but not others / "No space left on device"

You may have hit the inotify watch limit. [Here's a summary of what this means
and how to increase it.][inotify limit]

[inotify limit]: https://watchexec.github.io/docs/inotify-limits.html

### Docker: it's not responding correctly to signal or has trouble managing processes

Cargo Watch (and Watchexec underlying) does not currently support running as PID 1.
It will probably work for basic uses, but you should consider using a supervisor,
init, or shell to handle PID 1 concerns. With Docker, the `--init` option may be useful.

See [watchexec#140](https://github.com/watchexec/watchexec/issues/140) for more.

### Docker: running cargo commands over a mount is very slow

This isn't really a Cargo Watch issue, but when your host system is not Linux,
running commands from inside the container on a volume or bind mount from the
host will perform very badly due to filesystem indirection. Consider [building
outside the mount if possible][i-219]:

```dockerfile
# ...
RUN mkdir -p /build
WORKDIR `/src`
ENTRYPOINT cargo watch -C /build --manifest-path=/src/Cargo.toml -- cargo run
```

Or similarly with [`CARGO_TARGET_DIR`](https://doc.rust-lang.org/cargo/reference/config.html#buildtarget-dir).

```dockerfile
# ...
RUN mkdir -p /build
WORKDIR `/src`
ENV CARGO_TARGET_DIR=/build
ENTRYPOINT cargo watch -- cargo run
```

[i-219]: https://github.com/watchexec/cargo-watch/issues/219

You may also have issues where it's the file updates that aren't triggering in
a timely manner, not the compilation taking a long time. In that case, you
should run Cargo Watch or [Watchexec] _outside_ of Docker, on the host, and
signal the container for restart or reload.

### If you want to only recompile one Cargo workspace member crate

Watching one or more specific workspace member [is not natively supported yet][i-52],
although you can use `-w folder` to approximate it.

Watching the entire workspace and running a command in one member is done via
the usual `-p` option _on the child command_:

```
$ cargo watch -x 'build -p subcrate'
```

[i-52]: https://github.com/watchexec/cargo-watch/issues/52

### If it runs repeatedly without touching anything

That can happen when watching files that are modified by the command you're
running.

If you're only running compiles or checks (i.e. any command that only affects
the target/ folder) and you're using `-w`, you might be confusing the
target-folder-ignorer. Check your options and paths.

You can also use the `--watch-when-idle` flag to ignore any event that happens
while the command is running.

### If it runs repeatedly only touching ignored files

Make sure the files you ignored are the only ones being touched. Use the
`--why` option to see exactly which files were modified and triggered the
restart. Some programs and libraries create temporary files
that may not match a simple ignore pattern.

As above, you can also use the `--watch-when-idle` flag to help.

### I don't have colour in my cargo output / for cargo test

This sometimes happens on some terminal configurations or for test harnesses.
A quick workaround (instead of going down the rabbit hole of debugging your
console settings) is to pass `--color=always` to the command. E.g.

```
$ cargo watch -x 'check --color=always'
```

For test (and bench) commands, you'll need to pass the flag to the underlying
program instead of cargo:

```
$ cargo watch -x 'test -- --color=always'
```

### I want to compile my build with additional features

```
$ cargo watch --features foo,bar
```

will run `cargo check --features foo,bar` on every watched change.

The `--features` will be passed to every supported `cargo` subcommand.

```
$ cargo watch --features foo,bar -x build -x doc
```

will run both `build` and `doc` with the `foo` and `bar` features.

### Something not covered above / I have a feature request

Please [open an issue][watch-issues], or look through the existing ones. You
may also want to look through [issues for the Notify library][notify-issues]
this tool depends on, or the [issues for the Watchexec tool][watchexec-issues]
that we use under the covers (where I am also a maintainer).

If you want more verbose output, try running with the `--debug` flag. Note that
this will also enable debug mode for watchexec. When filing an issue, **make
sure to include a log with `--debug` enabled so problems can be diagnosed.**

**If your issue is a watchexec issue, open it there directly.** If you're not
sure, feel free to open it here, but if it _is_ a watchexec issue, it will get
closed in favour of the upstream issue.

[notify-issues]: https://github.com/notify-rs/notify/issues
[watch-issues]: https://github.com/watchexec/cargo-watch/issues
[watchexec-issues]: https://github.com/watchexec/watchexec/issues

### I want to embed Cargo Watch in my own (Rust) tool

It is not recommended to do that directly. You may of course call `cargo-watch`
as any other program, and technically it exposes an (undocumented) library that
could be directly / statically embedded. If you have no other option, that may
be your best bet.

However, for most cases, consider building on top of [Watchexec] instead. That
is itself built on [Notify], and both of these can be used as Rust libraries.

- If you want to build a tool that runs, restarts, and otherwise manages
  commands in response to file changes, you'll most probably want to use
  **Watchexec**.

- If you want to build a tool that responds to file changes, but does not need
  to run commands, or does so in a way that is not well-supported by Watchexec,
  then **Notify** is your ticket.

[Notify]: https://github.com/notify-rs/notify
[Watchexec]: https://github.com/watchexec/watchexec

### Wait, is this just a wrapper on top of watchexec?

Kind of! [Watchexec] does a really good job of watching files and running
commands and all the details that go with this. Cargo Watch uses the Watchexec
library interface and calls it with its own custom options, defaults, and
particularities, so you can just run `cargo-watch` in your project and be in
business.

When asking questions and/or filing bugs, keep in mind that Cargo Watch and
Watchexec share the same maintainer at the moment (but Notify does not,
anymore)!

## About

Created by [Félix Saparelli][passcod] and [awesome contributors][contributors].

[contributors]: https://github.com/watchexec/cargo-watch/network/members
[passcod]: https://passcod.name
