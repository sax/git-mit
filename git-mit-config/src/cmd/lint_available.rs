use std::env::current_dir;

use clap::{Arg, ArgMatches, Command};
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{external, lints::read_from_toml_or_else_vcs};
use mit_lint::Lints;

use crate::get_vcs;

pub fn cli<'help>() -> Command<'help> {
    Command::new("available")
        .arg(
            Arg::new("scope")
                .long("scope")
                .short('s')
                .possible_values(&["local", "global"])
                .default_value("local"),
        )
        .about("List the available lints")
}

pub fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("available").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<()> {
    let subcommand = matches
        .subcommand_matches("lint")
        .and_then(|matches| matches.subcommand_matches("available"))
        .unwrap();
    let is_local = Some("local") == subcommand.value_of("scope");
    let current_dir = current_dir().into_diagnostic()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;

    let lints = read_from_toml_or_else_vcs(&toml, &mut vcs)?;
    mit_commit_message_lints::console::style::lint_table(Lints::available(), &lints);

    Ok(())
}
