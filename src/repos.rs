use crate::errors;
use regex::Regex;

use errors::{AppResult, Error};

pub struct RepoMeta {
    pub protocol: String,
    pub owner: String,
    pub repo: String,
}

pub fn parse_repo_input(repo_input: &str) -> AppResult<RepoMeta> {
    let re = Regex::new(
        r"(?x)
            (?P<protocol>(git@|https://))?
            (?P<host>([\w\.@]+))?
            (/|:|^)
            (?P<owner>[\w,\-,_]+)
            /
            (?P<repo>[\w,\-,_]+)
            (.git)?/?
            ",
    )
    .unwrap();

    if re.is_match(repo_input) {
        let caps = re.captures(repo_input).unwrap();
        let protocol = match caps.name("protocol") {
            Some(p) => p.as_str().to_owned(),
            None => "https".into(),
        };

        if caps.name("host").is_some() && caps.name("host").unwrap().as_str() != "github.com" {
            return Err(Error::BadHost);
        }

        let owner = match caps.name("owner") {
            Some(o) => o.as_str().to_owned(),
            //TODO: handle this error
            None => return Err(Error::BadOwner),
        };

        let repo = match caps.name("repo") {
            Some(r) => r.as_str().to_owned(),
            //TODO: handle this error
            None => return Err(Error::BadRepo),
        };

        Ok(RepoMeta {
            protocol,
            owner,
            repo,
        })
    } else {
        return Err(Error::BadInput);
    }
}
