use crate::errors;
use errors::AppResult;
use git2::Repository;

pub fn default_branch(path: &str) -> AppResult<String> {
    match Repository::open(path) {
        Ok(r) => Ok(r
            .head()
            .unwrap()
            .name()
            .unwrap()
            .split("/")
            .last()
            .unwrap()
            .into()),
        Err(_) => Ok("main".into()),
    }
}
