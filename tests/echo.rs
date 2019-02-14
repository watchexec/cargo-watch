extern crate assert_cmd;
extern crate cargo_watch;
#[macro_use]
extern crate insta;
extern crate wait_timeout;
extern crate watchexec;

use assert_cmd::prelude::*;
use std::{
    fs::OpenOptions,
    io,
    path::PathBuf,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};
use wait_timeout::ChildExt;

fn touch(n: u8) -> io::Result<()> {
    let path: PathBuf = format!("./tests/touchdata/{}.txt", n).into();
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .map(|_| ())
}

fn std_to_string<T: io::Read>(handle: &mut Option<T>) -> String {
    if let Some(ref mut handle) = handle {
        let mut buf = String::with_capacity(1024);
        handle.read_to_string(&mut buf).unwrap();
        buf
    } else {
        unreachable!()
    }
}

// fsevents has trouble
#[cfg(not(target_os = "macos"))]
#[test]
fn without_poll() {
    let mut main = Command::cargo_bin("cargo-watch")
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
    touch(0).unwrap();

    if main.wait_timeout(Duration::from_secs(30)).unwrap().is_none() {
        main.kill().unwrap();
    }

    main.wait_with_output().unwrap().assert().success();
}

#[test]
fn with_poll() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "watch",
            "--testing-only--once",
            "--no-gitignore",
            "--poll",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo it runs",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(50));
    touch(1).unwrap();

    if main.wait_timeout(Duration::from_secs(30)).unwrap().is_none() {
        main.kill().unwrap();
    }

    main.wait_with_output().unwrap().assert().success();
}

#[test]
fn with_announce() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "watch",
            "--testing-only--once",
            "--no-gitignore",
            "--poll",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo with announce",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(50));
    touch(2).unwrap();

    if main.wait_timeout(Duration::from_secs(30)).unwrap().is_none() {
        main.kill().unwrap();
    }

    assert_snapshot_matches!("with_announce.stderr", std_to_string(&mut main.stderr));
    assert_snapshot_matches!("with_announce.stdout", std_to_string(&mut main.stdout));
}

#[test]
fn without_announce() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "watch",
            "--testing-only--once",
            "--no-gitignore",
            "--quiet",
            "--poll",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo without announce",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(50));
    touch(3).unwrap();

    if main.wait_timeout(Duration::from_secs(30)).unwrap().is_none() {
        main.kill().unwrap();
    }

    assert_snapshot_matches!("without_announce.stderr", std_to_string(&mut main.stderr));
    assert_snapshot_matches!("without_announce.stdout", std_to_string(&mut main.stdout));
}

#[test]
fn without_watch() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "--testing-only--once",
            "--no-gitignore",
            "--quiet",
            "--poll",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo without watch",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(50));
    touch(3).unwrap();

    if main.wait_timeout(Duration::from_secs(30)).unwrap().is_none() {
        main.kill().unwrap();
    }

    assert_snapshot_matches!("without_watch.stderr", std_to_string(&mut main.stderr));
    assert_snapshot_matches!("without_watch.stdout", std_to_string(&mut main.stdout));
}

#[cfg(unix)]
#[test]
fn with_error() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "watch",
            "--testing-only--once",
            "--no-gitignore",
            "--poll",
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
    touch(4).unwrap();

    if main.wait_timeout(Duration::from_secs(30)).unwrap().is_none() {
        main.kill().unwrap();
    }

    assert_snapshot_matches!("with_error.stderr", std_to_string(&mut main.stderr));
    assert_snapshot_matches!("with_error.stdout", std_to_string(&mut main.stdout));
}
