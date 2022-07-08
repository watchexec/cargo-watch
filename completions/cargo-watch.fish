complete -c cargo-watch -l delay-run -d 'Sleep some time before running commands' -r
complete -c cargo-watch -l quit-after-n -d 'Quit after a set amount of triggers' -r
complete -c cargo-watch -l features -d 'Feature(s) passed to cargo invocations' -r
complete -c cargo-watch -s x -l exec -d 'Cargo command(s) to execute on changes' -r
complete -c cargo-watch -s s -l shell -d 'Shell command(s) to execute on changes' -r
complete -c cargo-watch -s d -l delay -d 'File updates debounce delay' -r
complete -c cargo-watch -s i -l ignore -d 'Ignore a path pattern' -r
complete -c cargo-watch -s p -l package -d 'Reserved for workspace support' -r
complete -c cargo-watch -s w -l watch -d 'Watch specific file(s) or folder(s)' -r
complete -c cargo-watch -s S -l use-shell -d 'Shell to use for --shell commands, or `none` for direct execution' -r
complete -c cargo-watch -s C -l workdir -d 'Change working directory of the command' -r
complete -c cargo-watch -s E -l env -d 'Inject environment variables into the commands\' environments' -r
complete -c cargo-watch -s B -d 'Inject RUST_BACKTRACE=value into the commands\' environments' -r
complete -c cargo-watch -s L -d 'Inject RUST_LOG=value into the commands\' environments' -r
complete -c cargo-watch -s h -l help -d 'Show the help'
complete -c cargo-watch -s V -l version -d 'Show the version'
complete -c cargo-watch -s c -l clear -d 'Clear the screen before each run'
complete -c cargo-watch -l debug -d 'Show debug output'
complete -c cargo-watch -l why -d 'Show paths that changed'
complete -c cargo-watch -l ignore-nothing -d 'Ignore nothing, not even target/ and .git/'
complete -c cargo-watch -l no-gitignore -d 'Don’t use .gitignore files'
complete -c cargo-watch -l no-ignore -d 'Don’t use .ignore files'
complete -c cargo-watch -l no-restart -d 'Don’t restart command while it’s still running'
complete -c cargo-watch -l all -d 'Reserves for workspace support'
complete -c cargo-watch -l poll -d 'Force use of polling for file changes'
complete -c cargo-watch -l postpone -d 'Postpone first run until a file changes'
complete -c cargo-watch -s q -l quiet -d 'Suppress output from cargo watch itself'
complete -c cargo-watch -s N -l notify -d 'Send a desktop notification on command start and end'
complete -c cargo-watch -l no-auto-env -d 'Don’t inject CARGO_WATCH_* variables in the environment'
