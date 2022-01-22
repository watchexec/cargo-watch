# Cargo Watch Troubleshooting

In all cases, start by checking your version with `cargo watch --version` and,
if necessary, upgrading to [the latest one][cw-downloads].

[cw-downloads]: https://watchexec.github.io/downloads/cargo-watch

## RLS is slow while using cargo watch, or vice versa, or it's waiting for the project lock a lot

Cargo builds (and checks, and clippy, and tests because the tests have to be
built) take out a lock on the project so two cargo instances don't run at the
same time.

However, Rust Analyzer is much better at this, so use that instead of RLS.

## I'm getting errors on Windows 7 (or lower)

Cargo Watch versions 5.0.0 and up (and Watchexec versions 1.3.0 and up) **[do
not support Windows 7 or lower][i-69].** Support _will not_ be added. Issues for
Windows <=7 will be closed. If it works, lucky you, but that is not intentional.

[i-69]: https://github.com/watchexec/cargo-watch/issues/69

## I want to run cargo-watch directly, without going through cargo

You can! But you'll have to specify the `watch` subcommand as the first
argument, like so:

```
$ cargo-watch watch -x build
```

## I want to run cargo-watch outside of a Cargo project

That's not supported. If you have a good reason to use a Cargo-specific tool
outside a Cargo project, please open an issue! Otherwise, you'll probably be
best served with using [Watchexec].

[Watchexec]: https://watchexec.github.io

## If file updates seems to never trigger

Try using `--poll` to force the polling fallback.

If that still doesn't work, and you're using an editor that does "safe saving",
like IntelliJ / PyCharm, you may have to disable "safe saving" as that may
prevent file notifications from being generated properly.

Also try using the `--why` option to see if the paths you expect are changing.

## Linux: If it fails to watch some deep directories but not others / "No space left on device"

You may have hit the inotify watch limit. [Here's a summary of what this means
and how to increase it.][inotify limit]

[inotify limit]: https://watchexec.github.io/docs/inotify-limits.html

## If you want to only recompile one Cargo workspace member crate

Watching one or more specific workspace member [is not natively supported yet][i-52],
although you can use `-w folder` to approximate it.

Watching the entire workspace and running a command in one member is done via
the usual `-p` option _on the child command_:

```
$ cargo watch -x 'build -p subcrate'
```

[i-52]: https://github.com/watchexec/cargo-watch/issues/52

## If it runs repeatedly without touching anything

That can happen when watching files that are modified by the command you're
running.

If you're only running compiles or checks (i.e. any command that only affects
the target/ folder) and you're using `-w`, you might be confusing the
target-folder-ignorer. Check your options and paths.

You can also use the `--watch-when-idle` flag to ignore any event that happens
while the command is running. **This will become the default in 8.0.**

## If it runs repeatedly only touching ignored files

Make sure the files you ignored are the only ones being touched. Use the
`--why` option to see exactly which files were modified and triggered the
restart. Some programs and libraries create temporary files
that may not match a simple ignore pattern.

As above, you can also use the `--watch-when-idle` flag to help.

## I don't have colour in my cargo output / for cargo test

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

## I want to compile my build with additional features

```
$ cargo watch --features foo,bar
```

will run `cargo check --features foo,bar` on every watched change.

The `--features` will be passed to every supported `cargo` subcommand.

```
$ cargo watch --features foo,bar -x build -x doc
```

will run both `build` and `doc` with the `foo` and `bar` features.

## Something not covered above / I have a feature request

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
