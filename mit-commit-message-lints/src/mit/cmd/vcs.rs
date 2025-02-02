use std::borrow::Cow;

use miette::{Result, WrapErr};

use crate::external::Vcs;

#[allow(clippy::maybe_infinite_iter)]
pub fn get_vcs_coauthors_config<'a>(
    config: &'a dyn Vcs,
    key: &'a str,
) -> Result<Vec<Option<Cow<'a, str>>>> {
    (0..)
        .take_while(|index| has_vcs_coauthor(config, *index))
        .map(|index| get_vcs_coauthor_config(config, key, index))
        .fold(Ok(Vec::<Option<Cow<'a, str>>>::new()), |acc, item| {
            match (acc, item) {
                (Err(error), _) | (Ok(_), Err(error)) => Err(error).wrap_err("failed to read "),
                (Ok(list), Ok(item)) => Ok(vec![list, vec![item]].concat()),
            }
        })
}

pub fn has_vcs_coauthor(config: &dyn Vcs, index: i32) -> bool {
    let email = get_vcs_coauthor_config(config, "email", index);
    let name = get_vcs_coauthor_config(config, "name", index);

    matches!((name, email), (Ok(Some(_)), Ok(Some(_))))
}

pub fn get_vcs_coauthor_config<'a>(
    config: &'a dyn Vcs,
    key: &'a str,
    index: i32,
) -> Result<Option<Cow<'a, str>>> {
    config
        .get_str(&format!("mit.author.coauthors.{}.{}", index, key))
        .map(|x| x.map(std::convert::Into::into))
}
