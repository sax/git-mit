use std::time::Duration;

use clap::ArgMatches;

use crate::errors::GitRelatesTo;

pub struct Args {
    matches: ArgMatches,
}

impl From<ArgMatches> for Args {
    fn from(matches: ArgMatches) -> Self {
        Self { matches }
    }
}
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::console::completion::Shell;

impl Args {
    pub(crate) fn issue_number(&self) -> Result<&str> {
        match self.matches.value_of("issue-number") {
            None => Err(GitRelatesTo::NoRelatesToMessageSet.into()),
            Some(value) => Ok(value),
        }
    }

    pub(crate) fn timeout(&self) -> Result<Duration> {
        match self.matches.value_of("timeout") {
            None => Err(GitRelatesTo::NoTimeoutSet.into()),
            Some(value) => Ok(value),
        }
        .and_then(|timeout| timeout.parse().into_diagnostic())
        .map(|timeout: u64| timeout * 60)
        .map(Duration::from_secs)
    }

    pub fn completion(&self) -> Option<Shell> {
        self.matches.value_of_t::<Shell>("completion").ok()
    }
}
