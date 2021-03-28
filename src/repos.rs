use crate::errors;
use regex::Regex;

use errors::{AppResult, Error};

#[derive(Debug, PartialEq)]
pub struct RepoMeta {
    pub protocol: String,
    pub url_stem: String,
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
    .expect("Regular expression invalid.");

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
            None => return Err(Error::BadOwner),
        };

        let repo = match caps.name("repo") {
            Some(r) => r.as_str().to_owned(),
            None => return Err(Error::BadRepo),
        };

        Ok(RepoMeta {
            protocol,
            url_stem: format!("{}/{}", owner, repo),
        })
    } else {
        Err(Error::BadInput)
    }
}

#[test]
fn it_parses_stem_repos() {
    let repo = "bradyjoslin/sharewifi";
    let repo_meta = parse_repo_input(repo).unwrap();

    assert_eq!(
        repo_meta,
        RepoMeta {
            protocol: "https".into(),
            url_stem: "bradyjoslin/sharewifi".into(),
        }
    );
}

#[test]
fn it_parses_full_repos() {
    let repo = "https://github.com/bradyjoslin/sharewifi";
    let repo_meta = parse_repo_input(repo).unwrap();

    assert_eq!(
        repo_meta,
        RepoMeta {
            protocol: "https://".into(),
            url_stem: "bradyjoslin/sharewifi".into(),
        }
    );
}

#[test]
fn it_parses_git_repos() {
    let repo = "git@github.com:bradyjoslin/sharewifi.git";
    let repo_meta = parse_repo_input(repo).unwrap();

    assert_eq!(
        repo_meta,
        RepoMeta {
            protocol: "git@".into(),
            url_stem: "bradyjoslin/sharewifi".into(),
        }
    );
}

#[test]
fn it_only_parses_github_git_repos() {
    let repo = "git@githubs.com:bradyjoslin/sharewifi.git";
    let repo_meta = parse_repo_input(repo);

    assert_eq!(repo_meta.is_err(), true);
}

#[test]
fn it_only_parses_github_http_repos() {
    let repo = "https://githubs.com/bradyjoslin/sharewifi";
    let repo_meta = parse_repo_input(repo);

    assert_eq!(repo_meta.is_err(), true);
}
