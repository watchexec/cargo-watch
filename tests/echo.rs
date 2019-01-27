extern crate cargo_watch;
extern crate watchexec;
#[macro_use]
extern crate insta;
extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::{
    fs::OpenOptions,
    io,
    path::PathBuf,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

fn touch(n: u8) -> io::Result<()> {
    let path: PathBuf = format!("./tests/touchdata/{}.txt", n).into();
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .map(|_| ())
}

#[test]
fn it_runs() {
    let main = Command::main_binary()
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "watch",
            "--testing-only--once",
            "--no-gitignore",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo it runs",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(50));
    touch(1).unwrap();

    main.wait_with_output().unwrap().assert().success();
}

#[test]
fn with_announce() {
    let main = Command::main_binary()
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "watch",
            "--testing-only--once",
            "--no-gitignore",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo with announce",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(50));
    touch(1).unwrap();

    let out = main.wait_with_output().unwrap();

    assert_snapshot_matches!(
        "with_announce.stdout",
        std::str::from_utf8(&out.stdout).unwrap()
    );
    assert_snapshot_matches!(
        "with_announce.stderr",
        std::str::from_utf8(&out.stderr).unwrap()
    );
}

#[test]
fn without_announce() {
    let main = Command::main_binary()
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "watch",
            "--testing-only--once",
            "--no-gitignore",
            "--quiet",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo without announce",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(50));
    touch(1).unwrap();

    let out = main.wait_with_output().unwrap();

    assert_snapshot_matches!(
        "without_announce.stdout",
        std::str::from_utf8(&out.stdout).unwrap()
    );
    assert_snapshot_matches!(
        "without_announce.stderr",
        std::str::from_utf8(&out.stderr).unwrap()
    );
}

#[cfg(unix)]
#[test]
fn with_error() {
    let main = Command::main_binary()
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "watch",
            "--testing-only--once",
            "--no-gitignore",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo with error",
            "-s",
            "false",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(50));
    touch(1).unwrap();

    let out = main.wait_with_output().unwrap();

    assert_snapshot_matches!(
        "with_error.stdout",
        std::str::from_utf8(&out.stdout).unwrap()
    );
    assert_snapshot_matches!(
        "with_error.stderr",
        std::str::from_utf8(&out.stderr).unwrap()
    );
}
