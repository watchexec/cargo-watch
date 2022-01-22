use clap::IntoApp;
use clap_complete::{generate_to, shells::*, Generator};
use clap_complete_fig::Fig;
use std::io::Error;

#[path = "../src/args.rs"]
mod args;

fn main() -> Result<(), Error> {
		gen_help(Bash)?;
		gen_help(Elvish)?;
		gen_help(Fig)?;
		gen_help(Fish)?;
		gen_help(PowerShell)?;
		gen_help(Zsh)?;

	Ok(())
}

fn gen_help(gen: impl Generator) -> Result<(), Error> {
	let mut app = args::App::into_app();
	let path = generate_to(gen, &mut app, "cargo-watch", "./completions/")?;
	println!("completion file is generated: {:?}", path);
	Ok(())
}
