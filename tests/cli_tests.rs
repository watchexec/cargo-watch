use std::{env, path::PathBuf};

use trycmd::{cargo::cargo_bin, schema::Bin, TestCases};

#[test]
#[cfg(unix)]
fn unix_tests() {
	TestCases::new()
		.default_bin_path(cargo_bin!("cargo-watch"))
		.register_bin(
			"test-cw",
			Bin::Path(
				PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
					.join("target/debug/examples/test-cw"),
			),
		)
		.case("tests/trycmd/unix/*.trycmd");
}
