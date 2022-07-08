
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'cargo-watch' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'cargo-watch'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'cargo-watch' {
            [CompletionResult]::new('--delay-run', 'delay-run', [CompletionResultType]::ParameterName, 'Sleep some time before running commands')
            [CompletionResult]::new('--quit-after-n', 'quit-after-n', [CompletionResultType]::ParameterName, 'Quit after a set amount of triggers')
            [CompletionResult]::new('--features', 'features', [CompletionResultType]::ParameterName, 'Feature(s) passed to cargo invocations')
            [CompletionResult]::new('-x', 'x', [CompletionResultType]::ParameterName, 'Cargo command(s) to execute on changes')
            [CompletionResult]::new('--exec', 'exec', [CompletionResultType]::ParameterName, 'Cargo command(s) to execute on changes')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Shell command(s) to execute on changes')
            [CompletionResult]::new('--shell', 'shell', [CompletionResultType]::ParameterName, 'Shell command(s) to execute on changes')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'File updates debounce delay')
            [CompletionResult]::new('--delay', 'delay', [CompletionResultType]::ParameterName, 'File updates debounce delay')
            [CompletionResult]::new('-i', 'i', [CompletionResultType]::ParameterName, 'Ignore a path pattern')
            [CompletionResult]::new('--ignore', 'ignore', [CompletionResultType]::ParameterName, 'Ignore a path pattern')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Reserved for workspace support')
            [CompletionResult]::new('--package', 'package', [CompletionResultType]::ParameterName, 'Reserved for workspace support')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Watch specific file(s) or folder(s)')
            [CompletionResult]::new('--watch', 'watch', [CompletionResultType]::ParameterName, 'Watch specific file(s) or folder(s)')
            [CompletionResult]::new('-S', 'S', [CompletionResultType]::ParameterName, 'Shell to use for --shell commands, or `none` for direct execution')
            [CompletionResult]::new('--use-shell', 'use-shell', [CompletionResultType]::ParameterName, 'Shell to use for --shell commands, or `none` for direct execution')
            [CompletionResult]::new('-C', 'C', [CompletionResultType]::ParameterName, 'Change working directory of the command')
            [CompletionResult]::new('--workdir', 'workdir', [CompletionResultType]::ParameterName, 'Change working directory of the command')
            [CompletionResult]::new('-E', 'E', [CompletionResultType]::ParameterName, 'Inject environment variables into the commands'' environments')
            [CompletionResult]::new('--env', 'env', [CompletionResultType]::ParameterName, 'Inject environment variables into the commands'' environments')
            [CompletionResult]::new('-B', 'B', [CompletionResultType]::ParameterName, 'Inject RUST_BACKTRACE=value into the commands'' environments')
            [CompletionResult]::new('-L', 'L', [CompletionResultType]::ParameterName, 'Inject RUST_LOG=value into the commands'' environments')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Show the help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Show the help')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Show the version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Show the version')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Clear the screen before each run')
            [CompletionResult]::new('--clear', 'clear', [CompletionResultType]::ParameterName, 'Clear the screen before each run')
            [CompletionResult]::new('--debug', 'debug', [CompletionResultType]::ParameterName, 'Show debug output')
            [CompletionResult]::new('--why', 'why', [CompletionResultType]::ParameterName, 'Show paths that changed')
            [CompletionResult]::new('--ignore-nothing', 'ignore-nothing', [CompletionResultType]::ParameterName, 'Ignore nothing, not even target/ and .git/')
            [CompletionResult]::new('--no-gitignore', 'no-gitignore', [CompletionResultType]::ParameterName, 'Don’t use .gitignore files')
            [CompletionResult]::new('--no-ignore', 'no-ignore', [CompletionResultType]::ParameterName, 'Don’t use .ignore files')
            [CompletionResult]::new('--no-restart', 'no-restart', [CompletionResultType]::ParameterName, 'Don’t restart command while it’s still running')
            [CompletionResult]::new('--all', 'all', [CompletionResultType]::ParameterName, 'Reserves for workspace support')
            [CompletionResult]::new('--poll', 'poll', [CompletionResultType]::ParameterName, 'Force use of polling for file changes')
            [CompletionResult]::new('--postpone', 'postpone', [CompletionResultType]::ParameterName, 'Postpone first run until a file changes')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Suppress output from cargo watch itself')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Suppress output from cargo watch itself')
            [CompletionResult]::new('-N', 'N', [CompletionResultType]::ParameterName, 'Send a desktop notification on command start and end')
            [CompletionResult]::new('--notify', 'notify', [CompletionResultType]::ParameterName, 'Send a desktop notification on command start and end')
            [CompletionResult]::new('--no-auto-env', 'no-auto-env', [CompletionResultType]::ParameterName, 'Don’t inject CARGO_WATCH_* variables in the environment')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
