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

To upgrade:

    $ cargo install --force cargo-watch

Or clone and build with `$ cargo build` then place in your $PATH.

## Usage

By default, it runs `check` (which is available [since Rust 1.16][st-check]).
You can easily override this, though:

    $ cargo watch [command...]

A few examples:

    $ cargo watch test
    $ cargo watch run
    $ cargo watch doc
    $ cargo watch test bench
    $ cargo watch "build --release"
    $ cargo watch "build --release" "test test_"

[st-check]: https://blog.rust-lang.org/2017/03/16/Rust-1.16.html

### Force polling

If the commands are never triggering, or you're getting this error:

    ERROR:cargo_watch: Failed to init notify

You can try the alternative (polling) file watcher, by passing `--poll`.

### Clear screen

If you prefer to clear the screen before running commands, you can pass the
`--clear` flag. If you want to clear the screen in-between individual commands,
you can use `clear` as a command, e.g.

    $ cargo watch check clear test

### Cargo run

~~Cargo Watch has special behaviour with `run` commands: it will restart the
process on file change. This works especially well when developing servers
or other applications that never return on normal operation.~~

⚠ This currently doesn't work properly, see [#25](https://github.com/passcod/cargo-watch/issues/25). ⚠

As a result of this long-standing issue (a contributed fix would be immensely
appreciated, but I'll get to it eventually), if you're developing servers it's
probably better to use an alternative, like [nodemon] if you have Node, or
[watchexec] if you like Rust tooling.

[watchexec]: https://github.com/mattgreen/watchexec

## Details and tips

It pairs well with [dybuk], the compiler output prettifier:

    $ cargo watch |& dybuk

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

It [does not yet][i-52] support Cargo workspaces.

[i-52]: https://github.com/passcod/cargo-watch/issues/52

## Etc

Created by [Félix Saparelli][passcod] and [awesome contributors][contributors].

[contributors]: https://github.com/passcod/cargo-watch/network/members
[dybuk]: https://github.com/Ticki/dybuk
[notify]: https://github.com/passcod/rsnotify
[passcod]: https://passcod.name
