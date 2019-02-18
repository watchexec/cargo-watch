extern crate assert_cmd;
extern crate cargo_watch;
extern crate wait_timeout;
extern crate watchexec;

use assert_cmd::prelude::*;
use std::{
    process::{Command, Stdio},
    time::Duration,
};
use wait_timeout::ChildExt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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

    let assert = main.wait_with_output().unwrap().assert().success();
    assert.stdout(format!("cargo-watch {}\n", VERSION));
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

    let assert = main.wait_with_output().unwrap().assert().success();
    assert.stdout(format!("cargo-watch {}\n", VERSION));
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

    let assert = main.wait_with_output().unwrap().assert().success();
    assert.stdout(format!("cargo-watch {}\n", VERSION));
}
