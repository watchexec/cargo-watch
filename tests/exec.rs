extern crate assert_cmd;
extern crate cargo_watch;
extern crate predicates;
extern crate wait_timeout;
extern crate watchexec;

use assert_cmd::prelude::*;
use predicates::str::is_match;
use std::{
    process::{Command, Stdio},
    time::Duration,
};
use wait_timeout::ChildExt;

#[test]
fn with_cargo() {
    let mut main = Command::new("cargo")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&["watch", "--version"])
        .spawn()
        .unwrap();

    if main.wait_timeout(Duration::from_secs(1)).unwrap().is_none() {
        main.kill().unwrap();
    }

    main.wait_with_output()
        .unwrap()
        .assert()
        .success()
        .stdout(is_match(r"cargo-watch \d+\.\d+\.\d+\n").unwrap());
}

#[test]
fn without_cargo() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&["watch", "--version"])
        .spawn()
        .unwrap();

    if main.wait_timeout(Duration::from_secs(1)).unwrap().is_none() {
        main.kill().unwrap();
    }

    main.wait_with_output()
        .unwrap()
        .assert()
        .success()
        .stdout(is_match(r"cargo-watch \d+\.\d+\.\d+\n").unwrap());
}

#[test]
fn without_watch() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&["--version"])
        .spawn()
        .unwrap();

    if main.wait_timeout(Duration::from_secs(1)).unwrap().is_none() {
        main.kill().unwrap();
    }

    main.wait_with_output()
        .unwrap()
        .assert()
        .success()
        .stdout(is_match(r"cargo-watch \d+\.\d+\.\d+\n").unwrap());
}
