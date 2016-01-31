# $ cargo watch

[![Crate release version](https://img.shields.io/crates/v/cargo-watch.svg?style=flat-square)](https://crates.io/crates/cargo-watch)
[![Crate license: CC0 1.0](https://img.shields.io/crates/l/cargo-watch.svg?style=flat-square)](https://creativecommons.org/publicdomain/zero/1.0/)
![Crate download count](https://img.shields.io/crates/d/cargo-watch.svg?style=flat-square)

[![Build status (Travis)](https://img.shields.io/travis/passcod/cargo-watch.svg?style=flat-square)](https://travis-ci.org/passcod/cargo-watch)
[![Code of Conduct](https://img.shields.io/badge/contributor-covenant-123456.svg?style=flat-square)](http://contributor-covenant.org/version/1/1/0/)

Cargo Watch watches over your project's source for changes, and runs Cargo
commands when they occur.

If you've used [nodemon], [gulp], [guard], [watchman], or similar others,
it will probably feel familiar.

[nodemon]: http://nodemon.io/
[gulp]: http://gulpjs.com/
[guard]: http://guardgem.org/
[watchman]: https://facebook.github.io/watchman/

## Install

    $ cargo install cargo-watch

Or clone and build with `$ cargo build` then place in your $PATH.

## Upgrade

Cargo has no easy upgrade mechanism at the moment, so you'll need to:

    $ cargo uninstall cargo-watch
    $ cargo install cargo-watch

## Usage

By default, it runs `test` (which implies `build`).
You can easily override this, though:

    $ cargo watch [command...]

A few examples:

    $ cargo watch doc
    $ cargo watch test bench
    $ cargo watch "build --release"
    $ cargo watch "build --release" "test test_"

### Cargo run

Cargo Watch has special behaviour with `run` commands: it will restart the
process on file change. This works especially well when developing servers
or other applications that never return on normal operation.

## Details and tips

It pairs well with [dybuk], the compiler output prettifier:

    $ cargo watch check |& dybuk

Just like any Cargo command, it will run from any project subdirectory.

Cargo Watch will ignore everything that's not a Rust file, and files that start
with either a dot (`.foo.rs`) or a tilde (`~foo.rs`).

It uses the [notify] crate for file events, so it supports all platforms, some
more efficiently than others (if you use the big three — Linux, Mac, Windows —
you will be fine).

If your Cargo Watch fails to watch some deep directories but not others, and you
are on Linux, you may have hit [the inotify watch limit](http://blog.sorah.jp/2012/01/24/inotify-limitation).
You can either increase the limit (instructions are on the previous link and at
[this Guard wiki page](https://github.com/guard/listen/wiki/Increasing-the-amount-of-inotify-watchers)),
or you can stop whatever it is that's consuming so many inotify watches.

## Etc

Created by [Félix Saparelli][passcod] and [awesome contributors][contributors].

[contributors]: https://github.com/passcod/cargo-watch/network/members
[dybuk]: https://github.com/Ticki/dybuk
[notify]: https://github.com/passcod/rsnotify
[passcod]: https://passcod.name
