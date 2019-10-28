use watchexec::{
    cli::Args,
    error::Result,
    pathop::PathOp,
    run::{ExecHandler, Handler},
};

pub struct CwHandler<'a> {
    args: &'a Args,
    quiet: bool,
    inner: ExecHandler<'a>,
}

impl<'a> Handler for CwHandler<'a> {
    fn args(&self) -> Args {
        self.inner.args()
    }

    fn on_manual(&self) -> Result<bool> {
        if self.args.once {
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

impl<'a> CwHandler<'a> {
    pub fn new(watchexec_args: &'a mut Args, quiet: bool) -> Result<CwHandler> {
        let mut final_cmd = watchexec_args.cmd.join(" && ");
        if !quiet {
            #[cfg(unix)]
            final_cmd.push_str("; echo [Finished running. Exit status: $?]");
            #[cfg(windows)]
            final_cmd.push_str(" & echo [Finished running. Exit status: %ERRORLEVEL%]");
            #[cfg(not(any(unix, windows)))]
            final_cmd.push_str(" ; echo [Finished running]");
            // ^ could be wrong depending on the platform, to be fixed on demand
        }

        watchexec_args.cmd = vec![final_cmd];

        Ok(CwHandler {
            args: watchexec_args,
            inner: ExecHandler::new(watchexec_args)?,
            quiet,
        })
    }

    fn start(&self) {
        if !self.quiet {
            println!("[Running '{}']", self.args.cmd.join(" && "));
        }
    }
}
