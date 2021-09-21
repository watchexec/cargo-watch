# $ cargo watch

[![Crate release version](https://flat.badgen.net/crates/v/cargo-watch)](https://crates.io/crates/cargo-watch)
[![Crate license: CC0 1.0](https://flat.badgen.net/github/license/watchexec/cargo-watch)](https://creativecommons.org/publicdomain/zero/1.0/)
[![Crate download count](https://flat.badgen.net/crates/d/cargo-watch)](https://crates.io/crates/cargo-watch)
[![CI status](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml/badge.svg)](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml)
[![MSRV: 1.51.0](https://flat.badgen.net/badge/MSRV/1.51.0/purple)](https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html)
![MSRV policy: bump is non-breaking](https://flat.badgen.net/badge/MSRV%20policy/non-breaking/orange)
[![Uses Caretaker Maintainership](https://flat.badgen.net/badge/Caretaker/Maintainership%20👥%20/purple)][caretaker]

Cargo Watch watches over your project's source for changes, and runs Cargo
commands when they occur.

If you've used [nodemon], [guard], or [entr], it will probably feel familiar.

[nodemon]: http://nodemon.io/
[entr]: https://github.com/eradman/entr
[guard]: http://guardgem.org/

- In the public domain / licensed with CC0.
- Uses [Caretaker Maintainership][caretaker].
- Website and more documentation: **[watchexec.github.io](https://watchexec.github.io)**.
- Minimum Supported Rust Version: 1.51.0.

[caretaker]: ./CARETAKERS.md

## Install

Pre-built binaries are available **[from the website][cw-downloads]** or
alternatively [on the Github Releases tab][releases]. Since 7.8.0, checksums and
signatures are also provided; see [download documentation][downloads] for details.

[cw-downloads]: https://watchexec.github.io/downloads/cargo-watch
[downloads]: https://watchexec.github.io/downloads/
[releases]: https://github.com/watchexec/cargo-watch/releases

```
$ cargo install cargo-watch
```

With [cargo-binstall](https://github.com/ryankurte/cargo-binstall):

```
$ cargo binstall cargo-watch
```

Or clone and build with `$ cargo build` then place in your $PATH.

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
        --no-gitignore       Don’t use .gitignore files
        --no-ignore          Don’t use .ignore files
        --no-restart         Don’t restart command while it’s still running
        --poll               Force use of polling for file changes
        --postpone           Postpone first run until a file changes
    -V, --version            Display version information
        --watch-when-idle    Ignore events emitted while the commands run.
                             Will become default behaviour in 8.0.

OPTIONS:
    -x, --exec <cmd>...
            Cargo command(s) to execute on changes [default: check]

    -s, --shell <cmd>...           Shell command(s) to execute on changes

    -d, --delay <delay>
            File updates debounce delay in seconds [default: 0.5]

        --features <features>
            List of features passed to cargo invocations

    -i, --ignore <pattern>...      Ignore a glob/gitignore-style pattern

    -B <rust-backtrace>
            Inject RUST_BACKTRACE=VALUE (generally you want to set it to 1)
            into the environment

        --use-shell <use-shell>
            Use a different shell. E.g. --use-shell=bash. On Windows, try
            --use-shell=powershell, which will become the default in 8.0.

    -w, --watch <watch>...
            Watch specific file(s) or folder(s) [default: .]

    -C, --workdir <workdir>
            Change working directory before running command [default: crate root]

ARGS:
    <cmd:trail>...    Full command to run. -x and -s will be ignored!

Cargo commands (-x) are always executed before shell commands (-s). You can use
the `-- command` style instead, note you'll need to use full commands, it won't
prefix `cargo` for you.

By default, your entire project is watched, except for the target/ and .git/
folders, and your .ignore and .gitignore files are used to filter paths.

On Windows, patterns given to -i have forward slashes (/) automatically
converted to backward ones (\) to ease command portability.
```

### Ignore files

`.gitignore` files are used by default to ignore paths to watch and trigger
runs. To stop honouring them, pass `--no-gitignore`.

`.ignore` files in the same syntax are also used by default. This file can be
used to specify files that should be ignored by cargo watch but checked into
git, without constantly adding `--ignore abc` options on the command-line. Do
note that `.ignore` files may also be used by other programs, like
[ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md#automatic-filtering).
To stop honouring these, pass `--no-ignore`.

Cargo watch also has an internal list of default ignores on top of those
specified in files, like `target/` and `.git/` and various other common types
(logs, editor swap files, lockfiles, etc).

To skip absolutely all ignores, use the `--ignore-nothing` flag.

`.git/info/exclude` and the global `$HOME/.gitignore` and similar ignore files
[are not supported yet][w-58].

[w-58]: https://github.com/watchexec/watchexec/issues/58

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

[Brought up by @LeDominik](https://github.com/watchexec/cargo-watch/issues/75),
here's a pattern that may be very useful: you're working on a server or app,
but want it to keep running while you're writing a new feature or fixing a bug,
potentially causing the code not to compile anymore in the meantime.

In this case, you can use this strategy: run a first `cargo watch` with check,
build, test, or whatever you want, and append `-s 'touch .trigger` (or equivalent
for your platform). Then, run a second `cargo watch` simultaneously that _only_
watches that `.trigger` file. For example:

```
$ cargo watch -x check -s 'touch .trigger'
```

and

```
$ cargo watch --no-gitignore -w .trigger -x run
```

The `--no-gitignore` flag ensures that you can safely add `.trigger` to your
`.gitignore` file to avoid mistakenly committing it.

## Troubleshooting

In all cases, start by checking your version with `cargo watch --version` and,
if necessary, upgrading to [the latest one][cw-downloads].

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
while the command is running. **This will become the default in 8.0.**

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
[Watchexec]: https://watchexec.github.io

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
