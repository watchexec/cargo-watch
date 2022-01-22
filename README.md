# $ cargo watch

[![Crate release version](https://flat.badgen.net/crates/v/cargo-watch)](https://crates.io/crates/cargo-watch)
[![Crate license: CC0 1.0](https://flat.badgen.net/github/license/watchexec/cargo-watch)](https://creativecommons.org/publicdomain/zero/1.0/)
[![Crate download count](https://flat.badgen.net/crates/d/cargo-watch)](https://crates.io/crates/cargo-watch)
[![CI status](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml/badge.svg)](https://github.com/watchexec/cargo-watch/actions/workflows/check.yml)
[![MSRV: 1.58.0](https://flat.badgen.net/badge/MSRV/1.58.0/purple)](https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html)
![MSRV policy: bump is non-breaking](https://flat.badgen.net/badge/MSRV%20policy/non-breaking/orange)

Cargo Watch watches over your project's source for changes, and runs Cargo
commands when they occur.

If you've used [nodemon], [guard], or [entr], it will probably feel familiar.

[nodemon]: http://nodemon.io/
[entr]: https://github.com/eradman/entr
[guard]: http://guardgem.org/

- In the public domain / licensed with CC0.
- Uses [Caretaker Maintainership][caretaker].
- Website and more documentation: **[watchexec.github.io](https://watchexec.github.io)**.
- Minimum Supported Rust Version: 1.58.0.

[caretaker]: ./CARETAKERS.md

## Install

<a href="https://repology.org/project/cargo-watch/versions"><img align="right" src="https://repology.org/badge/vertical-allrepos/cargo-watch.svg" alt="Packaging status"></a>

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

There's a lot more you can do! Check out:

- [Usage guide](./USAGE.md)
- [Manual page](./cargo-watch.1.ronn)
- [Troubleshooting](./TROUBLESHOOT.md)

## About

Created by [Félix Saparelli][passcod] and [awesome contributors][contributors].

[contributors]: https://github.com/watchexec/cargo-watch/network/members
[passcod]: https://passcod.name
