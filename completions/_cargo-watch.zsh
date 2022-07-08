#compdef cargo-watch

autoload -U is-at-least

_cargo-watch() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'--delay-run=[Sleep some time before running commands]:seconds: ' \
'--quit-after-n=[Quit after a set amount of triggers]:number: ' \
'*--features=[Feature(s) passed to cargo invocations]:FEATURES: ' \
'*-x+[Cargo command(s) to execute on changes]:cmd: ' \
'*--exec=[Cargo command(s) to execute on changes]:cmd: ' \
'*-s+[Shell command(s) to execute on changes]:cmd: ' \
'*--shell=[Shell command(s) to execute on changes]:cmd: ' \
'-d+[File updates debounce delay]:DELAY: ' \
'--delay=[File updates debounce delay]:DELAY: ' \
'*-i+[Ignore a path pattern]:pattern: ' \
'*--ignore=[Ignore a path pattern]:pattern: ' \
'*-p+[Reserved for workspace support]:spec: ' \
'*--package=[Reserved for workspace support]:spec: ' \
'*-w+[Watch specific file(s) or folder(s)]:path: ' \
'*--watch=[Watch specific file(s) or folder(s)]:path: ' \
'*-S+[Shell to use for --shell commands, or `none` for direct execution]:shell: ' \
'*--use-shell=[Shell to use for --shell commands, or `none` for direct execution]:shell: ' \
'-C+[Change working directory of the command]:path: ' \
'--workdir=[Change working directory of the command]:path: ' \
'*-E+[Inject environment variables into the commands'\'' environments]:key=value: ' \
'*--env=[Inject environment variables into the commands'\'' environments]:key=value: ' \
'-B+[Inject RUST_BACKTRACE=value into the commands'\'' environments]:RUST_BACKTRACE value: ' \
'-L+[Inject RUST_LOG=value into the commands'\'' environments]:RUST_LOG value: ' \
'-h[Show the help]' \
'--help[Show the help]' \
'-V[Show the version]' \
'--version[Show the version]' \
'-c[Clear the screen before each run]' \
'--clear[Clear the screen before each run]' \
'--debug[Show debug output]' \
'--why[Show paths that changed]' \
'--ignore-nothing[Ignore nothing, not even target/ and .git/]' \
'--no-gitignore[Don’t use .gitignore files]' \
'--no-ignore[Don’t use .ignore files]' \
'--no-restart[Don’t restart command while it’s still running]' \
'--all[Reserves for workspace support]' \
'--poll[Force use of polling for file changes]' \
'--postpone[Postpone first run until a file changes]' \
'-q[Suppress output from cargo watch itself]' \
'--quiet[Suppress output from cargo watch itself]' \
'-N[Send a desktop notification on command start and end]' \
'--notify[Send a desktop notification on command start and end]' \
'--no-auto-env[Don’t inject CARGO_WATCH_* variables in the environment]' \
'*::cmd-trail -- Full command to run. -x and -s will be ignored!:' \
&& ret=0
}

(( $+functions[_cargo-watch_commands] )) ||
_cargo-watch_commands() {
    local commands; commands=()
    _describe -t commands 'cargo-watch commands' commands "$@"
}

_cargo-watch "$@"
