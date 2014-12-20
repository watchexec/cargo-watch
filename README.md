# $ cargo watch

## Usage

1. Build with `$ cargo build`.
2. Place in your $PATH.
3. Invoke using `$ cargo watch`.

## What?

It will watch your `src` folder and any subdirectories for file changes,
additions, removals, and moves (in or out), and run both `$ cargo build`
and `$ cargo doc` on your project.

Just like any Cargo command, you can run it from any subdirectory in your
project tree and it will find its way.

It's hard-coded to not compile things more than once per 2 seconds, to avoid
overloading your computer. It will also ignore everything that's not a Rust
file, everything that's a dot-file, and cache/backup files (`.filename.swo`
and `~filename.rs`).

It uses the [notify](https://github.com/passcod/rsnotify) crate for file
events, so it supports Linux through inotify, and (untested) all other
platforms through polling (and native solutions once they get there).

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
