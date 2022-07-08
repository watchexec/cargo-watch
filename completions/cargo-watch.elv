
use builtin;
use str;

set edit:completion:arg-completer[cargo-watch] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'cargo-watch'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'cargo-watch'= {
            cand --delay-run 'Sleep some time before running commands'
            cand --quit-after-n 'Quit after a set amount of triggers'
            cand --features 'Feature(s) passed to cargo invocations'
            cand -x 'Cargo command(s) to execute on changes'
            cand --exec 'Cargo command(s) to execute on changes'
            cand -s 'Shell command(s) to execute on changes'
            cand --shell 'Shell command(s) to execute on changes'
            cand -d 'File updates debounce delay'
            cand --delay 'File updates debounce delay'
            cand -i 'Ignore a path pattern'
            cand --ignore 'Ignore a path pattern'
            cand -p 'Reserved for workspace support'
            cand --package 'Reserved for workspace support'
            cand -w 'Watch specific file(s) or folder(s)'
            cand --watch 'Watch specific file(s) or folder(s)'
            cand -S 'Shell to use for --shell commands, or `none` for direct execution'
            cand --use-shell 'Shell to use for --shell commands, or `none` for direct execution'
            cand -C 'Change working directory of the command'
            cand --workdir 'Change working directory of the command'
            cand -E 'Inject environment variables into the commands'' environments'
            cand --env 'Inject environment variables into the commands'' environments'
            cand -B 'Inject RUST_BACKTRACE=value into the commands'' environments'
            cand -L 'Inject RUST_LOG=value into the commands'' environments'
            cand -h 'Show the help'
            cand --help 'Show the help'
            cand -V 'Show the version'
            cand --version 'Show the version'
            cand -c 'Clear the screen before each run'
            cand --clear 'Clear the screen before each run'
            cand --debug 'Show debug output'
            cand --why 'Show paths that changed'
            cand --ignore-nothing 'Ignore nothing, not even target/ and .git/'
            cand --no-gitignore 'Don’t use .gitignore files'
            cand --no-ignore 'Don’t use .ignore files'
            cand --no-restart 'Don’t restart command while it’s still running'
            cand --all 'Reserves for workspace support'
            cand --poll 'Force use of polling for file changes'
            cand --postpone 'Postpone first run until a file changes'
            cand -q 'Suppress output from cargo watch itself'
            cand --quiet 'Suppress output from cargo watch itself'
            cand -N 'Send a desktop notification on command start and end'
            cand --notify 'Send a desktop notification on command start and end'
            cand --no-auto-env 'Don’t inject CARGO_WATCH_* variables in the environment'
        }
    ]
    $completions[$command]
}
