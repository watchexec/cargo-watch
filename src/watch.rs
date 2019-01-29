use watchexec::{
    cli::Args,
    error::Result,
    pathop::PathOp,
    run::{ExecHandler, Handler},
};

pub struct CwHandler {
    args: Args,
    quiet: bool,
    inner: ExecHandler,
}

impl Handler for CwHandler {
    fn new(mut args: Args) -> Result<Self> {
        let quiet = args.cmd.last() == Some(&"quiet".into());
        if quiet {
            args.cmd.pop();
        }

        let mut final_cmd = args.cmd.join(" && ");
        if !quiet {
            #[cfg(unix)]
            final_cmd.push_str("; echo [Finished running. Exit status: $?]");
            #[cfg(windows)]
            final_cmd.push_str(" & echo [Finished running. Exit status: %ERRORLEVEL%]");
            #[cfg(not(any(unix, windows)))]
            final_cmd.push_str(" ; echo [Finished running]");
            // ^ could be wrong depending on the platform, to be fixed on demand
        }

        let mut inner_args = args.clone();
        inner_args.cmd = vec![final_cmd];

        Ok(Self {
            args: args.clone(),
            quiet,
            inner: ExecHandler::new(inner_args)?,
        })
    }

    fn on_manual(&mut self) -> Result<bool> {
        self.start();
        self.inner.on_manual()
    }

    fn on_update(&mut self, ops: &[PathOp]) -> Result<bool> {
        self.start();
        self.inner.on_update(ops)
    }
}

impl CwHandler {
    fn start(&self) {
        if !self.quiet {
            println!("[Running '{}']", self.args.cmd.join(" && "));
        }
    }
}
