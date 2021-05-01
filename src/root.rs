use camino::Utf8PathBuf;
use clap::{Error, ErrorKind};
use log::debug;
use std::{env::set_current_dir, process::Command};

pub fn project_root() -> Utf8PathBuf {
    Command::new("cargo")
        .arg("locate-project")
        .arg("--message-format")
        .arg("plain")
        .output()
        .map_err(|err| err.to_string())
        .and_then(|out| String::from_utf8(out.stdout).map_err(|err| err.to_string()))
        .map(Utf8PathBuf::from)
        .and_then(|path| {
            path.parent()
                .ok_or_else(|| String::from("project root does not exist"))
                .map(ToOwned::to_owned)
        })
        .unwrap_or_else(|err| Error::with_description(&err, ErrorKind::Io).exit())
}

pub fn change_dir(dir: Utf8PathBuf) {
    debug!("change directory to: {}", dir);
    set_current_dir(dir)
        .unwrap_or_else(|err| Error::with_description(&err.to_string(), ErrorKind::Io).exit())
}
