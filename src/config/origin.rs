use std::{path::PathBuf, env};

use dunce::canonicalize;
use miette::{Result, IntoDiagnostic};
use project_origins::ProjectType;
use tracing::debug;
use watchexec::paths::common_prefix;

use crate::args::Args;

pub async fn dirs(args: &Args) -> Result<(PathBuf, PathBuf)> {
	let curdir = env::current_dir()
		.and_then(canonicalize)
		.into_diagnostic()?;
	debug!(?curdir, "current directory");

	let project_origin = if let Some(origin) = args.project_origin {
		debug!(?origin, "project origin override");
		canonicalize(origin).into_diagnostic()?
	} else {
		let homedir = dirs::home_dir()
			.map(canonicalize)
			.transpose()
			.into_diagnostic()?;
		debug!(?homedir, "home directory");

		let mut paths = HashSet::new();
		for path in args.values_of_os("paths").unwrap_or_default() {
			paths.insert(canonicalize(path).into_diagnostic()?);
		}

		let homedir_requested = homedir.as_ref().map_or(false, |home| paths.contains(home));
		debug!(
			?homedir_requested,
			"resolved whether the homedir is explicitly requested"
		);

		if paths.is_empty() {
			debug!("no paths, using current directory");
			paths.insert(curdir.clone());
		}

		debug!(?paths, "resolved all watched paths");

		let mut origins = HashSet::new();
		for path in paths {
			origins.extend(project_origins::origins(&path).await);
		}

		match (homedir, homedir_requested) {
			(Some(ref dir), false) if origins.contains(dir) => {
				debug!("removing homedir from origins");
				origins.remove(dir);
			}
			_ => {}
		}

		if origins.is_empty() {
			debug!("no origins, using current directory");
			origins.insert(curdir.clone());
		}

		debug!(?origins, "resolved all project origins");

		// This canonicalize is probably redundant
		canonicalize(
			common_prefix(&origins)
				.ok_or_else(|| miette!("no common prefix, but this should never fail"))?,
		)
		.into_diagnostic()?
	};
	info!(?project_origin, "resolved common/project origin");

	let workdir = curdir;
	info!(?workdir, "resolved working directory");

	Ok((project_origin, workdir))
}
