# $ cargo watch

[![Crate release version](https://flat.badgen.net/crates/v/cargo-watch)](https://crates.io/crates/cargo-watch)
[![Crate license: CC0 1.0](https://flat.badgen.net/github/license/passcod/cargo-watch)](https://creativecommons.org/publicdomain/zero/1.0/)
[![Crate download count](https://flat.badgen.net/crates/d/cargo-watch)](https://crates.io/crates/cargo-watch)
[![Build status](https://flat.badgen.net/travis/passcod/cargo-watch/master)](https://travis-ci.org/passcod/cargo-watch)
[![MSRV: 1.38.0](https://flat.badgen.net/badge/MSRV/1.38.0/purple)](https://blog.rust-lang.org/2019/09/26/Rust-1.38.0.html)
![MSRV policy: bump is non-breaking](https://flat.badgen.net/badge/MSRV%20policy/non-breaking/orange)

Cargo Watch watches over your project's source for changes, and runs Cargo
commands when they occur.

If you've used [nodemon], [gulp], [guard], [watchman], or similar others,
it will probably feel familiar.

[nodemon]: http://nodemon.io/
[gulp]: http://gulpjs.com/
[guard]: http://guardgem.org/
[watchman]: https://facebook.github.io/watchman/

## Install

Pre-built binaries are available [on the Github Releases tab](https://github.com/passcod/cargo-watch/releases).

```
$ cargo install cargo-watch
```

To upgrade:

```
$ cargo install --force cargo-watch
```

Or clone and build with `$ cargo build` then place in your $PATH.

## Usage

By default, it runs `check` (which is available [since Rust 1.16][st-check]).
You can easily override this, though:

```
$ cargo watch [-x command]...
```

[st-check]: https://blog.rust-lang.org/2017/03/16/Rust-1.16.html

A few examples:

```
# Run tests only
$ cargo watch -x test

# Run check then tests
$ cargo watch -x check -x test

# Run run with arguments
$ cargo watch -x 'run -- --some-arg'

# Run an arbitrary command
$ cargo watch -s 'echo Hello world'

# Run with features passed to cargo
$ cargo watch --features "feature1,feature2"
```

There's a lot more you can do! Here's a copy of the help:

```
USAGE:
    cargo watch [FLAGS] [OPTIONS]

FLAGS:
    -c, --clear             Clear the screen before each run
        --debug             Display debug output
    -h, --help              Display this message
        --ignore-nothing    Ignore nothing, not even target/ and .git/
        --no-gitignore      Don’t use .gitignore files
        --no-ignore         Don’t use .ignore files
        --no-restart        Don’t restart command while it’s still running
        --poll              Force use of polling for file changes
        --postpone          Postpone first run until a file changes
    -q, --quiet             Suppress output from cargo-watch itself
    -V, --version           Display version information
        --watch-when-idle   Ignore events emitted while the commands run

OPTIONS:
    -x, --exec <cmd>...         Cargo command(s) to execute on changes [default: check]
    -s, --shell <cmd>...        Shell command(s) to execute on changes
    -d, --delay <delay>         File updates debounce delay in seconds [default: 0.5]
        --features <features>   List of features passed to cargo invocations
    -i, --ignore <pattern>...   Ignore a glob/gitignore-style pattern
    -w, --watch <watch>...      Watch specific file(s) or folder(s) [default: .]

Cargo commands (-x) are always executed before shell commands (-s).

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

### Ignore syntax

See the [`glob::Pattern` docs][glob::Pattern] for a more detailed
specification of the glob matching syntax used for `--ignore`.

On Windows, patterns should be specified with Windows-style (`\\`) separators.
Unix-style separators (`/`) would not match Windows paths, which could be
confusing and give the appearance of commandline ignores not working.

From Cargo Watch 7.0.0, `/` in commandline ignores are automatically translated
to `\\` when running on Windows, but one should still try to write the correct
patterns for the platform, as there may be more subtle differences.

From Cargo Watch 7.3.0, `--ignore` patterns were fixed to provide better
experience with directory matching. Previously, ignoring a folder would need
unyieldy `-i folder/**` patterns; now that is handled internally, and only `-i
folder` is needed for the same effect.

[glob::Pattern]: https://doc.rust-lang.org/glob/glob/struct.Pattern.html

### Reloading servers seamlessly

Cargo Watch pairs very well with [Catflap], a tool for Unixy platforms that
lets one spawn a socket before the watcher runs that Rust servers can then bind
to, avoiding request-dropping and the infamous ADDRINUSE error. For example:

```
$ catflap -- cargo watch -x run
```

[Catflap]: https://github.com/passcod/catflap

Of course, if you don't need to guard against these issues or don't want to
modify your program to grab sockets instead of ports, you can use Cargo Watch
as-is: it will happily just restart your server normally.

### Restarting an application only if the build/check succeeds

[Brought up by @LeDominik](https://github.com/passcod/cargo-watch/issues/75),
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

## Contributing

The Cargo Watch team enthusiastically welcomes contributions and project
participation! There's a bunch of things you can do if you want to contribute!
The [Contributor Guide](./CONTRIBUTING.md) has all the information you need for
everything from reporting bugs to contributing entire new features. Please
don't hesitate to jump in if you'd like to, or even ask us questions if
something isn't clear. <sub>{[attribution](https://github.com/zkat/pacote#contributing)}</sub>

You can also contribute financially with [Patreon] or [GitHub Sponsorship].

[Patreon]: https://www.patreon.com/passcod
[GitHub Sponsors]: https://github.com/users/passcod/sponsorship

## Troubleshooting

In all cases, start by checking your version with `cargo watch --version` and,
if necessary, upgrading to [the latest one][releases].

[releases]: https://github.com/passcod/cargo-watch/releases

### RLS is slow while using cargo watch, or vice versa, or it's waiting for the project lock a lot

Cargo builds (and checks, and clippy, and tests because the tests have to be
built) take out a lock on the project so two cargo instances don't run at the
same time. That may include language servers like RLS (and possibly RA)! So
your editor might be fighting with cargo watch for the lock when you save.

There's not really a way around this. Either stop using cargo watch, or stop
using RLS/RA, or accept that whenever you save there may be some slowdown for a
little while. While I'm investigating ways to make this less of an issue,
there's not going to be a quick fix anytime soon. Rust Analyzer is itself
working on a different solution in conjunction with the compiler.

### On Windows 7 (or lower): "failed to add to job object: Access denied (OS Error 5)"

Cargo Watch versions 5.0.0 and up (and Watchexec versions 1.3.0 and up) [do not
support Windows 7 or lower][i-69]. There are no plans at the moment to add such
support.

You can downgrade to the last version which did support Windows 7 (and lower),
but do keep in mind that many bug fixes and features are missing there:

```
$ cargo install --force --vers 4.0.3 cargo-watch
```

[i-69]: https://github.com/passcod/cargo-watch/issues/69

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

### Linux: If it fails to watch some deep directories but not others / "No space left on device"

You may have hit the inotify watch limit. [Here's a summary of what this means
and how to increase it.][inotify limit]

[inotify limit]: https://blog.passcod.name/2017/jun/25/inotify-watch-limit

### If you want to only recompile one Cargo workspace

Cargo workspaces [are not natively supported yet][i-52].

However, as you can run "arbitrary commands" with the `-s` option, you can
write workspace-aware commands manually.

[i-52]: https://github.com/passcod/cargo-watch/issues/52

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
`--debug` option to see exactly which files were modified and triggered the
restart (or were ignored). Some programs and libraries create temporary files
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

The `--features` flag is recognized and passed to all cargo invocations.

```
$ cargo watch --features feature1,feature2
```

will run `cargo check --features feature1,feature2` on every watched change.

### Something not covered above / I have a feature request

Please [open an issue][watch-issues], or look through the existing ones. You
may also want to look through [issues for the Notify library][notify-issues]
this tool depends on, or the [issues for the Watchexec tool][watchexec-issues]
that we use under the covers.

If you want more verbose output, try running with the `--debug` flag. Note that
this will also enable debug mode for watchexec. When filing an issue, **make
sure to include a log with `--debug` enabled so problems can be diagnosed.**

[notify-issues]: https://github.com/passcod/notify/issues
[watch-issues]: https://github.com/passcod/cargo-watch/issues
[watchexec-issues]: https://github.com/mattgreen/watchexec/issues

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

[Notify]: https://github.com/passcod/notify
[Watchexec]: https://github.com/mattgreen/watchexec

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

[contributors]: https://github.com/passcod/cargo-watch/network/members
[passcod]: https://passcod.name
