#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::{convert::TryFrom, env, option::Option::None, time::Duration};

use git2::Repository;
use miette::Result;
use mit_commit_message_lints::{
    console::{exit::UnknownAuthor, style},
    external::Git2,
    mit::{set_commit_authors, Authors},
};

use crate::cli::args::Args;

mod cli;
mod config;
mod errors;

fn main() -> Result<()> {
    if env::var("DEBUG_PRETTY_ERRORS").is_ok() {
        miette::set_hook(Box::new(|_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .force_graphical(true)
                    .build(),
            )
        }))
        .unwrap();
    }

    let args: cli::args::Args = cli::app::app().get_matches().into();

    let mut git_config = Git2::try_from(Args::cwd()?)?;
    let file_authors = crate::config::author::load(&args)?;
    let authors = file_authors.merge(&Authors::try_from(&git_config)?);
    let initials = args.initials()?;

    if repo_present() && !is_hook_present() {
        not_setup_warning();
    };

    let missing = authors.missing_initials(initials.clone());

    if !missing.is_empty() {
        return Err(UnknownAuthor::new(
            &initials
                .clone()
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>(),
            missing.clone().into_iter().map(String::from).collect(),
        )
        .into());
    }

    set_commit_authors(
        &mut git_config,
        &authors.get(&initials),
        Duration::from_secs(args.timeout()? * 60),
    )?;

    Ok(())
}

fn not_setup_warning() {
    style::warning("Hooks not found in this repository, your commits won't contain trailers, and lints will not be checked", Some("git mit-install\n\nwill fix this"));
}

fn is_hook_present() -> bool {
    env::current_dir()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .map(|repo| repo.path().join("hooks").join("commit-msg"))
        .filter(|path_buf| match path_buf.canonicalize().ok() {
            None => false,
            Some(path) => path.to_string_lossy().contains("mit-commit-msg"),
        })
        .is_some()
}

fn repo_present() -> bool {
    env::current_dir()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .is_some()
}
