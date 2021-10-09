use std::{convert::TryFrom, env};

use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use miette::{GraphicalTheme, IntoDiagnostic, Result};
use mit_commit_message_lints::{
    console::style::print_completions,
    external::Git2,
    mit::{get_commit_coauthor_configuration, AuthorState},
};
use mit_commit_message_lints::console::style::miette_install;

use crate::{
    cli::app,
    errors::{NoAuthorError, StaleAuthorError},
};

fn main() -> Result<()> {
    miette_install();
    let mut app = app();
    let matches = app.clone().get_matches();

    // Simply print and exit if completion option is given.
    if let Some(completion) = matches.value_of("completion") {
        match completion {
            "bash" => print_completions::<Bash>(&mut app),
            "elvish" => print_completions::<Elvish>(&mut app),
            "fish" => print_completions::<Fish>(&mut app),
            "powershell" => print_completions::<PowerShell>(&mut app),
            "zsh" => print_completions::<Zsh>(&mut app),
            _ => println!("Unknown completion"), // Never reached
        }

        std::process::exit(0);
    }

    let current_dir = env::current_dir().into_diagnostic()?;
    let mut git_config = Git2::try_from(current_dir)?;
    let co_author_configuration = get_commit_coauthor_configuration(&mut git_config)?;

    if let AuthorState::Timeout(time) = co_author_configuration {
        return Err(StaleAuthorError::new(time).into());
    }

    if co_author_configuration.is_none() {
        return Err(NoAuthorError {}.into());
    }

    Ok(())
}

mod cli;
mod errors;
