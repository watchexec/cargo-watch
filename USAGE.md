# Cargo Watch Usage Guide

Compact help is available with `-h`, and longer descriptions with `--help`. On
supported systems, a manual page is also available. Shell completions should be
installed for Bash, Elvish, Fish, Powershell, and Zsh.

## Ignore files

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

## Ignore syntax

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

## Reloading servers seamlessly

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

## Restarting an application only if the build/check succeeds

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

## I want to embed Cargo Watch in my own (Rust) tool

It is not recommended to do that directly. You may of course call `cargo-watch`
as any other program. If you have no other option, that may be your best bet.

However, for most cases, consider building on top of [Watchexec] instead. That
is itself built on [Notify], and both of these can be used as Rust libraries.

[Notify]: https://github.com/notify-rs/notify
[Watchexec]: https://watchexec.github.io
