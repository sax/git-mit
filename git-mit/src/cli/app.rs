use clap::{crate_authors, crate_version, Arg, Command};
use indoc::indoc;

pub fn cli() -> Command<'static> {
    Command::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .after_help(indoc!(
            "
            COMMON TASKS:
                You can install git-mit into a new repository using

                    git mit-install

                You can add a new author to that repository by running

                    git mit-config mit set eg \"Egg Sample\" egg.sample@example.com

                You can save that author permanently by running

                    git mit-config mit set eg \"Egg Sample\" egg.sample@example.com
                    git mit-config mit generate > $HOME/.config/git-mit/mit.toml

                You can disable a lint by running

                    git mit-config lint disable jira-issue-key-missing

                You can install the example authors file to the default location with

                    git mit-config mit example > $HOME/.config/git-mit/mit.toml

                You can set the current author, and Co-authors by running

                    git mit ae se

                You can populate the `Relates-to` trailer using

                    git mit-relates-to \"[#12345678]\"
            "
        ))
        .arg(
            Arg::new("initials")
                .help("Initials of the mit to put in the commit")
                .multiple_values(true)
                .required_unless_present("completion"),
        )
        .arg(
            Arg::new("file")
                .short('c')
                .long("config")
                .help("Path to a file where mit initials, emails and names can be found")
                .env("GIT_MIT_AUTHORS_CONFIG")
                .takes_value(true)
                .default_value("$HOME/.config/git-mit/mit.toml"),
        )
        .arg(
            Arg::new("command")
                .short('e')
                .long("exec")
                .help(
                    "Execute a command to generate the mit configuration, stdout will be captured \
                     and used instead of the file, if both this and the file is present, this \
                     takes precedence",
                )
                .env("GIT_MIT_AUTHORS_EXEC")
                .takes_value(true),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .help("Number of minutes to expire the configuration in")
                .env("GIT_MIT_AUTHORS_TIMEOUT")
                .takes_value(true)
                .default_value("60"),
        )
        .arg(Arg::new("completion").long("completion").possible_values(&[
            "bash",
            "elvish",
            "fish",
            "powershell",
            "zsh",
        ]))
}
