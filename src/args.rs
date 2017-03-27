use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn parse() -> ArgMatches<'static> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .bin_name("cargo")
        .version(env!("CARGO_PKG_VERSION"))
        .help_message("")
        .version_message("")

        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::DontCollapseArgsInUsage)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::StrictUtf8)
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::SubcommandRequiredElseHelp)

        .subcommand(
            SubCommand::with_name("watch")
            .author(env!("CARGO_PKG_HOMEPAGE"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .usage("cargo watch [FLAGS] [OPTIONS]")

            .help_message("Display this message")
            .version_message("Display version information")

            .arg(Arg::with_name("clear")
                .short("c")
                .long("clear")
                .help("Clear the screen before each run"))

            .arg(Arg::with_name("poll")
                .long("poll")
                .help("Force use of polling for file changes"))

            .arg(Arg::with_name("cmd:cargo")
                .short("x")
                .long("exec")
                .takes_value(true)
                .value_name("cmd")
                .multiple(true)
                .empty_values(false)
                .min_values(1)
                .number_of_values(1)
                .default_value("check")
                .help("Cargo command(s) to execute on changes"))

            .arg(Arg::with_name("cmd:shell")
                .short("s")
                .long("shell")
                .takes_value(true)
                .value_name("cmd")
                .multiple(true)
                .empty_values(false)
                .min_values(1)
                .number_of_values(1)
                .help("Shell command(s) to execute on changes"))

            .arg(Arg::with_name("watch")
                .short("w")
                .long("watch")
                .takes_value(true)
                .multiple(true)
                .empty_values(false)
                .min_values(1)
                .number_of_values(1)
                .help("Watch specific file(s) or folder(s)"))

            .arg(Arg::with_name("delay")
                .short("d")
                .long("delay")
                .takes_value(true)
                .empty_values(false)
                .default_value("1")
                .help("File updates debounce delay in seconds"))

            .arg(Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Suppress output from cargo-watch itself"))

            .arg(Arg::with_name("postpone")
                .long("postpone")
                .help("Postpone first run until a file changes"))

            .arg(Arg::with_name("package")
                .short("p")
                .long("package")
                .takes_value(true)
                .value_name("spec")
                .multiple(true)
                .empty_values(false)
                .min_values(1)
                .hidden(true)
                .help("Reserved for workspace support"))

            .arg(Arg::with_name("all-packages")
                .long("all")
                .conflicts_with("package")
                .hidden(true)
                .help("Reserved for workspace support"))

            .after_help("Cargo commands (-x) are always executed before shell commands (-s).")

        ).get_matches();

    matches.subcommand.unwrap().matches
}
