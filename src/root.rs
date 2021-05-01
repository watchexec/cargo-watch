use camino::Utf8PathBuf;
use clap::{Error, ErrorKind};
use log::{debug};
use std::{env::set_current_dir, process::Command};

pub fn project_root() -> Result<Utf8PathBuf, Error> {
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
        .map_err(|err| Error::with_description(&err, ErrorKind::Io))
}

pub fn change_dir() {
    project_root()
        .and_then(|project_root| {
            debug!("change directory to cargo project root: {}", project_root);
            set_current_dir(project_root)
                .map_err(|err| Error::with_description(&err.to_string(), ErrorKind::Io))
        })
        .unwrap_or_else(|err| err.exit())
}
