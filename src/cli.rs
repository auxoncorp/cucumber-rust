use clap::{Arg, ArgAction, Command};

#[derive(Default)]
pub struct CliOptions {
    pub scenario_filter: Option<String>,
    pub nocapture: bool,
    pub debug: bool,
}

pub fn make_app() -> CliOptions {
    let matches = Command::new("cucumber")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Brendan Molloy <brendan@bbqsrc.net>")
        .about("Run the tests, pet a dog!")
        .arg(
            Arg::new("filter")
                .short('e')
                .long("expression")
                .value_name("regex")
                .help("Regex to select scenarios from"),
        )
        .arg(
            Arg::new("nocapture")
                .long("nocapture")
                .action(ArgAction::SetTrue)
                .help("Use this flag to disable suppression of output from tests"),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .action(ArgAction::SetTrue)
                .help("Enable verbose test logging (debug mode)"),
        )
        .get_matches();

    let nocapture = matches.contains_id("nocapture");
    let scenario_filter = matches.get_one::<String>("filter").cloned();
    let debug = matches.contains_id("debug");

    CliOptions {
        nocapture,
        scenario_filter,
        debug,
    }
}
