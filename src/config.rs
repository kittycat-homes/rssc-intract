use clap::{ArgMatches, Command};

/// parses command line arguments
pub fn cli_get_matches() -> ArgMatches {
    Command::new("rssc-intract")
        .about("an algorithm replacement")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("start"))
        .get_matches()
}
