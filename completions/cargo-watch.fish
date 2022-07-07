complete -c cargo-watch -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c cargo-watch -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c cargo-watch -n "__fish_use_subcommand" -f -a "watch" -d 'Watch your Cargo-based project and run commands when files change'
complete -c cargo-watch -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l features -d 'Feature(s) passed to cargo invocations' -r
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s x -l exec -d 'Cargo command(s) to execute on changes' -r
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s s -l shell -d 'Shell command(s) to execute on changes' -r
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s d -l delay -d 'File updates debounce delay' -r
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s i -l ignore -d 'Ignore a path pattern' -r
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s p -l package -d 'Reserved for workspace support' -r
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s w -l watch -d 'Watch specific file(s) or folder(s)' -r
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l use-shell -d 'Shell to use for --shell commands, or `none` for direct execution' -r
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s C -l workdir -d 'Change working directory of the command' -r
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l testing-only--once
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s c -l clear -d 'Clear the screen before each run'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l debug -d 'Show debug output'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l why -d 'Show paths that changed'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l ignore-nothing -d 'Ignore nothing, not even target/ and .git/'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l no-gitignore -d 'Don’t use .gitignore files'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l no-ignore -d 'Don’t use .ignore files'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l no-restart -d 'Don’t restart command while it’s still running'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l all -d 'Reserves for workspace support'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l poll -d 'Force use of polling for file changes'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -l postpone -d 'Postpone first run until a file changes'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s q -l quiet -d 'Suppress output from cargo watch itself'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s N -l notify -d 'Send a desktop notification on command start and end'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s B -d 'Inject RUST_BACKTRACE=value into the command\'s environment'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s L -d 'Inject RUST_LOG=value into the command\'s environment'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s h -l help -d 'Print help information'
complete -c cargo-watch -n "__fish_seen_subcommand_from watch" -s V -l version -d 'Print version information'
