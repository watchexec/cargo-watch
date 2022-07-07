# $ cargo watch

[![Crate release version](https://flat.badgen.net/crates/v/cargo-watch)](https://crates.io/crates/cargo-watch)
[![Crate license: Apache 2.0](https://flat.badgen.net/github/license/watchexec/cargo-watch)](https://www.apache.org/licenses/LICENSE-2.0.html)
[![Crate download count](https://flat.badgen.net/crates/d/cargo-watch)](https://crates.io/crates/cargo-watch)
[![CI status](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml/badge.svg)](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml)

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

Or with cargo (rustc >= 1.58.0) if you don't have Binstall yet:

```bash
$ cargo install cargo-watch
```

Or unpack directly from the [latest pre-built release](https://github.com/watchexec/cargo-watch/releases/latest).

This repository contains a [manual page](./cargo-watch.1) and [shell completions](./completions)
that you may want to install; the pre-built packages also include these.

## Usage

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

## About

Created by [Félix Saparelli][passcod] and [awesome contributors][contributors].

[contributors]: https://github.com/watchexec/cargo-watch/network/members
[passcod]: https://passcod.name
