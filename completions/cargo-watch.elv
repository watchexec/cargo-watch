
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
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand watch 'Watches over your Cargo project’s source'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'cargo-watch;watch'= {
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
            cand --use-shell 'Shell to use for the command, or `none` for direct execution'
            cand -C 'Change working directory of the command'
            cand --workdir 'Change working directory of the command'
            cand --testing-only--once 'testing-only--once'
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
            cand -B 'Inject RUST_BACKTRACE=value into the command''s environment'
            cand -L 'Inject RUST_LOG=value into the command''s environment'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
        }
        &'cargo-watch;help'= {
        }
    ]
    $completions[$command]
}
