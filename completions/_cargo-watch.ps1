
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
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'cargo-watch' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('watch', 'watch', [CompletionResultType]::ParameterValue, 'Watches over your Cargo project’s source')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cargo-watch;watch' {
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
            [CompletionResult]::new('--use-shell', 'use-shell', [CompletionResultType]::ParameterName, 'Shell to use for the command, or `none` for direct execution')
            [CompletionResult]::new('-C', 'C', [CompletionResultType]::ParameterName, 'Change working directory of the command')
            [CompletionResult]::new('--workdir', 'workdir', [CompletionResultType]::ParameterName, 'Change working directory of the command')
            [CompletionResult]::new('--testing-only--once', 'testing-only--once', [CompletionResultType]::ParameterName, 'testing-only--once')
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
            [CompletionResult]::new('-B', 'B', [CompletionResultType]::ParameterName, 'Inject RUST_BACKTRACE=value into the command''s environment')
            [CompletionResult]::new('-L', 'L', [CompletionResultType]::ParameterName, 'Inject RUST_LOG=value into the command''s environment')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            break
        }
        'cargo-watch;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
