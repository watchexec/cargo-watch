use watchexec::{
    config::Config,
    error::Result,
    pathop::PathOp,
    run::{ExecHandler, Handler},
};

pub struct CwHandler {
    cmd: String,
    once: bool,
    quiet: bool,
    notify: bool,
    inner: ExecHandler,
}

impl Handler for CwHandler {
    fn args(&self) -> Config {
        self.inner.args()
    }

    fn on_manual(&self) -> Result<bool> {
        if self.once {
            Ok(true)
        } else {
            self.start();
            self.inner.on_manual()
        }
    }

    fn on_update(&self, ops: &[PathOp]) -> Result<bool> {
        self.start();
        self.inner.on_update(ops).map(|o| {
            if self.notify {
                notify_rust::Notification::new()
                    .summary("Cargo Watch observed a change")
                    .body("Cargo Watch has seen a change, the command may have restarted.")
                    .show()
                    .map(drop)
                    .unwrap_or_else(|err| {
                        log::warn!("Failed to send desktop notification: {}", err);
                    });
            }

            o
        })
    }
}

impl CwHandler {
    pub fn new(mut args: Config, quiet: bool, notify: bool, trailing: bool) -> Result<Self> {
        let cmd = if trailing {
            args.cmd[0].clone()
        } else {
            let cmd = args.cmd.join(" && ");
            let mut final_cmd = cmd.clone();
            if !quiet {
                #[cfg(unix)]
                final_cmd.push_str(r#"; echo "[Finished running. Exit status: $?]""#);
                #[cfg(windows)]
                final_cmd.push_str(r#" & echo "[Finished running. Exit status: %ERRORLEVEL%]""#);
                #[cfg(not(any(unix, windows)))]
                final_cmd.push_str(r#" ; echo "[Finished running]""#);
                // ^ could be wrong depending on the platform, to be fixed on demand
            }

            args.cmd = vec![final_cmd];
            cmd
        };

        Ok(Self {
            once: args.once,
            cmd,
            inner: ExecHandler::new(args)?,
            quiet,
            notify,
        })
    }

    fn start(&self) {
        if !self.quiet {
            println!("[Running '{}']", self.cmd);
        }
    }
}
