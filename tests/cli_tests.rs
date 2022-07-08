use std::env::set_current_dir;

use duct::cmd;
use tempfile::TempDir;

#[test]
fn cli_tests() -> Box<dyn std::error::Error + Send> {
	let tmp_dir = TempDir::new()?;
	set_current_dir(tmp_dir.path())?;
	cmd!("cargo", "init", "--vcs", "git", "--bin").run()?;

	trycmd::TestCases::new().case("tests/cmd/*.trycmd");

	tmp_dir.close()?;
}
