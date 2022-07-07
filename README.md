# $ cargo watch

[![Crate release version](https://flat.badgen.net/crates/v/cargo-watch)](https://crates.io/crates/cargo-watch)
[![Crate license: Apache 2.0](https://flat.badgen.net/github/license/watchexec/cargo-watch)][license]
[![Crate download count](https://flat.badgen.net/crates/d/cargo-watch)](https://crates.io/crates/cargo-watch)
[![CI status](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml/badge.svg)](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml)
[![MSRV: 1.58.0](https://flat.badgen.net/badge/MSRV/1.58.0/purple)](https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html)

Cargo Watch watches over your project's source for changes, and runs Cargo
commands when they occur.

If you've used [nodemon], [guard], or [entr], it will probably feel familiar.

[nodemon]: http://nodemon.io/
[entr]: https://github.com/eradman/entr
[guard]: http://guardgem.org/

- License: [Apache 2.0][license].
- Website: **[watchexec.github.io](https://watchexec.github.io)**.
- Minimum Supported Rust Version: 1.58.0.

[license]: https://www.apache.org/licenses/LICENSE-2.0.html

## Install

<a href="https://repology.org/project/cargo-watch/versions"><img align="right" src="https://repology.org/badge/vertical-allrepos/cargo-watch.svg" alt="Packaging status"></a>

Pre-built binaries are available **[from the website][cw-downloads]** or
alternatively [on the Github Releases tab][releases].

[cw-downloads]: https://watchexec.github.io/downloads/cargo-watch
[releases]: https://github.com/watchexec/cargo-watch/releases

With [cargo-binstall](https://github.com/ryankurte/cargo-binstall):

```bash
$ cargo binstall cargo-watch
```

With cargo:

```bash
$ cargo install cargo-watch
```

From source:

```bash
# clone:
$ git clone https://github.com/watchexec/cargo-watch
$ cd cargo-watch

# build:
$ cargo build --release
$ target/release/cargo-watch -h

# or install:
$ cargo install --path .
$ cargo watch -h
```

This repository contains a [manual page](./cargo-watch.1) and
[shell completions](./completions) that you may want to install; the
pre-built packages also include these.

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
