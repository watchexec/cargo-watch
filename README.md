# $ cargo watch

[![Releases](https://img.shields.io/github/release/passcod/cargo-watch.svg?style=flat-square)](https://github.com/passcod/cargo-watch/releases)[![License](https://img.shields.io/crates/l/cargo-watch.svg?style=flat-square)](https://creativecommons.org/publicdomain/zero/1.0/)[![Travis](https://img.shields.io/travis/passcod/cargo-watch.svg?style=flat-square)](https://travis-ci.org/passcod/cargo-watch)[![Code of Conduct](https://img.shields.io/badge/contributor-covenant-123456.svg?style=flat-square)](http://contributor-covenant.org/version/1/1/0/)

## Usage

1. Build with `$ cargo build`.
2. Place in your $PATH.
3. Invoke using `$ cargo watch`.

![screenshot from 2014-12-21 15 09 10](https://cloud.githubusercontent.com/assets/155787/5516943/89478468-8923-11e4-89af-d0963542623d.png)

## What?

It will watch your `src` folder and any subdirectories for file changes,
additions, removals, and moves (in or out), and run both `$ cargo build` and
`$ cargo test` on your project.  You can also specify other things to be run,
e.g. `$ cargo doc` and `$ cargo bench`, by passing flags.
See `$ cargo watch --help` for more.

Just like any Cargo command, you can run it from any subdirectory in your
project tree and it will find its way.

It's hard-coded to not compile things more than once per 2 seconds, to avoid
overloading your computer. It will also ignore everything that's not a Rust
file, everything that's a dot-file, and cache/backup files (`.filename.swo`
and `~filename.rs`).

It uses the [notify](https://github.com/passcod/rsnotify) crate for file
events, so it supports Linux, OS X, and soon Windows through native APIs,
and has a polling implementation to fall back on for all other platforms.

## How?

It uses [notify](https://github.com/passcod/rsnotify) to watch files, and
simply runs `$ cargo <whatever>` as child processes.

## Why?

I was getting tired of having to switch windows / tmux panes to compile my
code. This is much faster, and because it shows the output of the command,
I can see compile errors and warnings with a save and fix them immediately.

## Who?

My name is Félix Saparelli a.k.a. [passcod](https://passcod.name). You can
find more about me on the internet.

Also a bunch of [awesome contributors][contributors] participated.

[contributors]: https://github.com/passcod/cargo-watch/network/members
