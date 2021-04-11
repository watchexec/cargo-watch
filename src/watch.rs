use watchexec::{
    error::Result,
    pathop::PathOp,
    run::{ExecHandler, Handler},
    config::Config,
};

pub struct CwHandler {
    cmd: String,
    once: bool,
    quiet: bool,
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
        self.inner.on_update(ops)
    }
}

impl CwHandler {
    pub fn new(mut args: Config, quiet: bool) -> Result<Self> {
        let cmd = args.cmd.join(" && ");
        let mut final_cmd = cmd.clone();
        if !quiet {
            #[cfg(unix)]
            final_cmd.push_str("; echo [Finished running. Exit status: $?]");
            #[cfg(windows)]
            final_cmd.push_str(" & echo [Finished running. Exit status: %ERRORLEVEL%]");
            #[cfg(not(any(unix, windows)))]
            final_cmd.push_str(" ; echo [Finished running]");
            // ^ could be wrong depending on the platform, to be fixed on demand
        }

        args.cmd = vec![final_cmd];

        Ok(Self {
            once: args.once,
            cmd,
            inner: ExecHandler::new(args)?,
            quiet,
        })
    }

    fn start(&self) {
        if !self.quiet {
            println!("[Running '{}']", self.cmd);
        }
    }
}
