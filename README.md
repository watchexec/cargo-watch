[![CI status](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml/badge.svg)](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml)

# Cargo Watch

Cargo Watch is a tool to watch your Cargo-based project and run commands when files change. It
focuses on the Rust development experience and aims to be flexible enough to suit most without
becoming complicated to use.

If you've used [nodemon], [guard], or [entr], it will probably feel familiar.

[nodemon]: http://nodemon.io/
[entr]: https://github.com/eradman/entr
[guard]: http://guardgem.org/

Looking for a similar tool that you can use for other kinds of projects?
Try [Watchexec](https://github.com/watchexec/watchexec), this project's bigger sibling.

## Install

<a href="https://repology.org/project/cargo-watch/versions"><img align="right" src="https://repology.org/badge/vertical-allrepos/cargo-watch.svg" alt="Packaging status"></a>

Install or upgrade today with [Binstall](https://github.com/ryankurte/cargo-binstall):

```bash
$ cargo binstall cargo-watch
```

Or with cargo (rustc >= 1.60.0) if you don't have Binstall yet:

```bash
$ cargo install cargo-watch
```

Or unpack directly from the [latest pre-built release](https://github.com/watchexec/cargo-watch/releases/latest).

This repository contains a [manual page](./cargo-watch.1) and [shell completions](./completions)
that you may want to install; the pre-built packages also include these.

## Use

By default, it runs `check`. You can easily override this, though:

```bash
$ cargo watch [-x command]...
```

A few examples:

```bash
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

There's a lot more you can do! Check out:

- [Usage guide](./USAGE.md)
- [Manual page](./cargo-watch.1.ronn)
- [Troubleshooting](./TROUBLESHOOT.md)

## Augment

Cargo Watch pairs well with:

- [just](https://github.com/casey/just): a modern alternative to `make`
- [systemfd](https://github.com/mitsuhiko/systemfd): socket-passing in development

## Extend

- [watchexec library](https://github.com/watchexec/watchexec/tree/main/crates/lib): the engine behind this tool.
- [clearscreen](https://github.com/watchexec/clearscreen): to clear the (terminal) screen on every platform.
- [command group](https://github.com/watchexec/command-group): to run commands in process groups.
- [ignore files](https://github.com/watchexec/watchexec/tree/main/crates/ignore-files): to find, parse, and interpret ignore files.
- [project origins](https://github.com/watchexec/watchexec/tree/main/crates/project-origins): to find the origin(s) directory of a project.
- [notify](https://github.com/notify-rs/notify): to respond to file modifications (third-party).
- [globset](https://crates.io/crates/globset): to match globs (third-party).
