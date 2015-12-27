# $ cargo watch

[![ Release](https://img.shields.io/github/tag/passcod/cargo-watch.svg?style=flat-square)](https://github.com/passcod/cargo-watch/releases)[![ License](https://img.shields.io/crates/l/cargo-watch.svg?style=flat-square)](https://creativecommons.org/publicdomain/zero/1.0/)[![ Travis](https://img.shields.io/travis/passcod/cargo-watch.svg?style=flat-square)](https://travis-ci.org/passcod/cargo-watch)[![ Code of Conduct](https://img.shields.io/badge/contributor-covenant-123456.svg?style=flat-square)](http://contributor-covenant.org/version/1/1/0/)

Cargo Watch watches over your project's source for changes, and runs Cargo
commands when they occur.

If you've used [nodemon], [gulp], [guard], [watchman], or similar others,
it will probably feel familiar.

[nodemon]: http://nodemon.io/
[gulp]: http://gulpjs.com/
[guard]: http://guardgem.org/
[watchman]: https://facebook.github.io/watchman/

## Usage

1. Build with `$ cargo build`.
2. Place in your $PATH.
3. Invoke using `$ cargo watch`.

You may also be able to use `cargo install`.

## Details

By default, it runs `build` then `test`. You can easily override this, though:

    $ cargo watch [command...]

A few examples:

```
$ cargo watch doc
$ cargo watch test bench
$ cargo watch "build --release"
$ cargo watch "build --release" "test test_"
```

It pairs well with [dybuk], the compiler output prettifier:

    $ cargo watch check |& dybuk

Just like any Cargo command, it will run from any project subdirectory.

Cargo Watch is currently hard-coded to not compile things more often than every
two seconds, to avoid overusage. If you wish to help implementing a better
solution, see [#2](https://github.com/passcod/cargo-watch/issues/2).

It will ignore everything that's not a Rust file, and files that start with
either a dot (`.foo.rs`) or a tilde (`~foo.rs`).

It uses the [notify] crate for file events, so it supports all platforms, some
more efficiently than others (if you use the big three — Linux, Mac, Windows —
you will be fine).

## Etc

Created by [Félix Saparelli][passcod] and [awesome contributors][contributors].

[contributors]: https://github.com/passcod/cargo-watch/network/members
[dybuk]: https://github.com/Ticki/dybuk
[notify]: https://github.com/passcod/rsnotify
[passcod]: https://passcod.name
