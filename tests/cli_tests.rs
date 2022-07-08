use std::{
	env::{current_dir, set_current_dir},
	path::PathBuf,
};

use duct::cmd;
use miette::{IntoDiagnostic, Result};
use tempfile::TempDir;
use trycmd::{cargo::cargo_bin, TestCases};

fn prepare() -> Result<(PathBuf, TempDir)> {
	let tests_dir = current_dir()
		.into_diagnostic()?
		.join("tests")
		.join("trycmd");

	let tmp_dir = TempDir::new().into_diagnostic()?;
	set_current_dir(tmp_dir.path()).into_diagnostic()?;
	cmd!(
		"cargo",
		"init",
		"--vcs",
		"git",
		"--bin",
		"--name",
		"cw-test",
		"--offline"
	)
	.run()
	.into_diagnostic()?;

	Ok((tests_dir, tmp_dir))
}

#[test]
#[cfg(unix)]
fn unix_tests() -> Result<()> {
	let (tests_dir, tmp_dir) = prepare()?;
	TestCases::new()
		.default_bin_path(cargo_bin!("cargo-watch"))
		.case(tests_dir.join("unix.trycmd"));
	tmp_dir.close().into_diagnostic()?;
	Ok(())
}
