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
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
":: :_cargo-watch_commands" \
"*::: :->cargo-watch" \
&& ret=0
    case $state in
    (cargo-watch)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:cargo-watch-command-$line[1]:"
        case $line[1] in
            (watch)
_arguments "${_arguments_options[@]}" \
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
'--use-shell=[Shell to use for the command, or `none` for direct execution]:shell: ' \
'-C+[Change working directory of the command]:path: ' \
'--workdir=[Change working directory of the command]:path: ' \
'--testing-only--once[]' \
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
'-B[Inject RUST_BACKTRACE=value into the command'\''s environment]' \
'-L[Inject RUST_LOG=value into the command'\''s environment]' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
'*::cmd-trail -- Full command to run. -x and -s will be ignored!:' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'*::subcommand -- The subcommand whose help message to display:' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_cargo-watch_commands] )) ||
_cargo-watch_commands() {
    local commands; commands=(
'watch:Watches over your Cargo project’s source' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'cargo-watch commands' commands "$@"
}
(( $+functions[_cargo-watch__help_commands] )) ||
_cargo-watch__help_commands() {
    local commands; commands=()
    _describe -t commands 'cargo-watch help commands' commands "$@"
}
(( $+functions[_cargo-watch__watch_commands] )) ||
_cargo-watch__watch_commands() {
    local commands; commands=()
    _describe -t commands 'cargo-watch watch commands' commands "$@"
}

_cargo-watch "$@"
