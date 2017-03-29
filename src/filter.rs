use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

pub struct Filter {
    matcher: Gitignore
}

impl Filter {
    pub fn create<P: AsRef<Path>>(root: P, more_patterns: Vec<String>, walktree: bool) -> Self {
        let mut builder = GitignoreBuilder::new(&root);

        if walktree {
            let walker = WalkDir::new(&root)
                .follow_links(true)
                .into_iter()
                .filter_entry(|e| not_in_target(&root, e))
                .filter_map(|e| e.ok().and_then(|e| only_gitignore(e)));

            info!("Walking tree for gitignores");
            for entry in walker {
                let path = entry.path();
                info!("Parsing gitignore: {}", path.display());
                if let Some(error) = builder.add(&path) {
                     warn!("Ignoring error adding gitignore: {}", path.display());
                     warn!("{}", error);
                }
            }
        }

        for pattern in more_patterns {
            info!("Adding extra pattern: {}", pattern);
            if let Err(error) = builder.add_line(None, &pattern) {
                 warn!("Ignoring error adding pattern: {}", pattern);
                 warn!("{}", error);
            }
        }

        let matcher = builder.build().unwrap_or_else(|e| {
            warn!("Couldn't build matcher: {}", e);
            info!("Falling back to empty matcher");
            Gitignore::empty()
        });

        info!("Built filter with {} patterns", matcher.len());
        Filter { matcher: matcher }
    }

    pub fn matched<P: AsRef<Path> + Debug>(&self, path: P) -> bool {
        info!("Filtering: {:?}", path);
        if self.matcher.is_empty() {
            info!("Filter is empty, not bothering to go further");
            return false;
        }

        let is_dir = fs::metadata(&path)
            .and_then(|m| Ok(m.is_dir()))
            .unwrap_or_else(|e| {
                warn!("Got error trying to resolve {:?}", path);
                warn!("{}", e);
                false
            });

        if is_dir {
            debug!("Decided that {:?} was a folder", path);
        }

        let matched = self.matcher.matched(&path, is_dir);
        debug!("Match data for file {:?}: {:?}", path, matched);
        match matched {
            Match::Ignore(_) => true,
            _ => false
        }
    }
}

fn not_in_target<P: AsRef<Path>>(root: P, entry: &DirEntry) -> bool {
    entry.path()
        .strip_prefix(&root)
        .ok()
        .map(|path| path.to_str())
        .map(|s| s.and_then(|p| {
            if p.starts_with("target/") {
                None
            } else {
                Some(())
            }
        }))
        .and_then(|r| Some(r.is_some()))
        .unwrap_or(false)
}

fn only_gitignore(entry: DirEntry) -> Option<DirEntry> {
    trace!("Is this a gitignore? {}", entry.path().display());
    if entry.file_name()
         .to_str()
         .map(|s| s == ".gitignore")
         .unwrap_or(false) {
        Some(entry)
    } else {
        None
    }
}
