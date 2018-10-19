# $ cargo watch

[![Crate release version](https://flat.badgen.net/crates/v/cargo-watch)](https://crates.io/crates/cargo-watch)
[![Crate license: CC0 1.0](https://flat.badgen.net/github/license/passcod/cargo-watch)](https://creativecommons.org/publicdomain/zero/1.0/)
[![Crate download count](https://flat.badgen.net/crates/d/cargo-watch)](https://crates.io/crates/cargo-watch)
[![Build status](https://flat.badgen.net/travis/passcod/cargo-watch/master)](https://travis-ci.org/passcod/cargo-watch)

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
        --no-restart        Don’t restart command while it’s still running
        --poll              Force use of polling for file changes
        --postpone          Postpone first run until a file changes
    -q, --quiet             Suppress output from cargo-watch itself
    -V, --version           Display version information

OPTIONS:
    -x, --exec <cmd>...
            Cargo command(s) to execute on changes [default: check]
    -s, --shell <cmd>...
            Shell command(s) to execute on changes
    -d, --delay <delay>
            File updates debounce delay in seconds [default: 0.5]
    -i, --ignore <pattern>...
            Ignore a glob/gitignore-style pattern
    -w, --watch <watch>...
            Watch specific file(s) or folder(s) [default: .]

Cargo commands (-x) are always executed before shell commands (-s).

By default, your entire project is watched, except for the target/
and .git/ folders, and your .gitignore files are used to filter paths.
```

### Ignore syntax

See the [`glob::Pattern` docs][glob::Pattern] for a more detailed
specification of the glob matching syntax used for `--ignore`.

On Windows, patterns should be specified with Windows-style (`\\`) separators.
Unix-style separators (`/`) would not match Windows paths, which could be
confusing and give the appearance of commandline ignores not working.

From Cargo Watch 7.0.0, `/` in commandline ignores are automatically translated
to `\\` when running on Windows, but one should still try to write the correct
patterns for the platform, as there may be more subtle differences.

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
$ cargo watch --no-gitignore -w .trigger -x run`
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

You can also contribute financially with [Ko-fi] or [Patreon].

[Ko-fi]: https://ko-fi.com/passcod
[Patreon]: https://www.patreon.com/passcod

## Troubleshooting

In all cases, start by checking your version with `cargo watch --version` and,
if necessary, upgrading to [the latest one][releases].

[releases]: https://github.com/passcod/cargo-watch/releases

### On Windows 7 (or lower): "failed to add to job object: Access denied (OS Error 5)"

Cargo Watch versions 5.0.0 and up (and watchexec versions 1.3.0 and up) [do not
support Windows 7 or lower][i-69]. There are no plans at the moment to add such
support.

You can downgrade to the last version which did support Windows 7 (and lower),
but do keep in mind that many bug fixes and features are missing there:

```
$ cargo install --force --vers 4.0.3 cargo-watch
```

[i-69]: https://github.com/passcod/cargo-watch/issues/69

### If running cargo watch errors with "Found argument 'build' which wasn't expected" (or similar)

You're probably using **version 4** (or higher) but using the **version 3** (or
lower) style of arguments. The interface changed! Refer to the sections above
for new usage guidelines, or to the help message:

```
$ cargo watch --help
```

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

You cannot do that directly. You may of course call `cargo-watch` as any other
program, but if you want to directly / statically embed it, that's not
possible. But! Cargo Watch is built on top of [Watchexec], Watchexec is itself
built on [Notify], and both of these can be used as Rust libraries.

- If you want to build a tool that runs, restarts, and otherwise manages
  commands in response to file changes, you'll most probably want to use
  **Watchexec**.

- If you want to build a tool that responds to file changes, but does not need
  to run commands, or does so in a way that is not well-supported by Watchexec,
  then **Notify** is your ticket.

[Notify]: https://github.com/passcod/notify
[Watchexec]: https://github.com/mattgreen/watchexec

### Wait, is this just a wrapper on top of watchexec?

It is! [Watchexec] does a really good job of watching files and running commands
and all the details that go with this. Cargo watch simply embeds watchexec and
calls it with its own custom options and defaults, so you can just run
`cargo-watch` in your project and be in business.

## About

Created by [Félix Saparelli][passcod] and [awesome contributors][contributors].

[contributors]: https://github.com/passcod/cargo-watch/network/members
[passcod]: https://passcod.name
