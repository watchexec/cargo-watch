extern crate assert_cmd;
extern crate cargo_watch;
extern crate wait_timeout;
extern crate watchexec;

use assert_cmd::prelude::*;
use assert_cmd::assert::Assert;
use std::{
    fs::OpenOptions,
    io,
    path::PathBuf,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};
use wait_timeout::ChildExt;

fn std_to_string<T: io::Read>(handle: &mut Option<T>) -> String {
    if let Some(ref mut handle) = handle {
        let mut buf = String::with_capacity(1024);
        handle.read_to_string(&mut buf).unwrap();
        buf
    } else {
        unreachable!()
    }
}

fn expect_version(assert: Assert) {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    assert.stdout(format!("cargo-watch {}\n", VERSION));
}

#[test]
fn with_cargo() {
    let mut main = Command::new("cargo")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "watch",
            "--version",
        ])
        .spawn()
        .unwrap();

    if main.wait_timeout(Duration::from_secs(1)).unwrap().is_none() {
        main.kill().unwrap();
    }

    expect_version(main.wait_with_output().unwrap().assert().success());
}

#[test]
fn without_cargo() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "watch",
            "--version",
        ])
        .spawn()
        .unwrap();

    if main.wait_timeout(Duration::from_secs(1)).unwrap().is_none() {
        main.kill().unwrap();
    }

    expect_version(main.wait_with_output().unwrap().assert().success());
}
