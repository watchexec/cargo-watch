use notify::{DebouncedEvent, PollWatcher, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::process::exit;
use std::sync::mpsc::Sender as SyncSender;
use std::time::Duration;

pub struct DualWatcher {
    primary: Option<RecommendedWatcher>,
    fallback: Option<PollWatcher>
}

pub type Sender = SyncSender<DebouncedEvent>;

impl DualWatcher {
    pub fn new(tx: Sender, d: Duration) -> Self {
        let primary = RecommendedWatcher::new(tx.clone(), d)
            .or_else(|e| {
                error!("Error initialising native notify, falling back to polling.");
                error!("{}", e);
                Err(())
            }).ok();

        let fallback = match primary {
            Some(_) => None,
            None => DualWatcher::fallback(tx, d)
        };

        if primary.is_none() && fallback.is_none() {
            exit(1);
        }

        DualWatcher { primary: primary, fallback: fallback }
    }

    fn fallback(tx: Sender, d: Duration) -> Option<PollWatcher> {
        PollWatcher::new(tx, d).or_else(|e| {
            error!("Error initialising fallback notify, aborting.");
            error!("{}", e);
            Err(())
        }).ok()
    }

    pub fn fallback_only(tx: Sender, d: Duration) -> Self {
        let fallback = DualWatcher::fallback(tx, d);
        if fallback.is_none() {
            exit(1);
        }

        DualWatcher { primary: None, fallback: fallback }
    }

    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let recurse = RecursiveMode::Recursive;
        match self.primary {
            Some(ref mut w) => w.watch(path, recurse),
            None => match self.fallback {
                Some(ref mut w) => w.watch(path, recurse),
                None => unreachable!()
            }
        }
    }
}
