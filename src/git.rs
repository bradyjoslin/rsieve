use crate::errors;
use errors::AppResult;
use git2::Repository;

pub fn default_branch(path: &str) -> AppResult<String> {
    Ok(Repository::discover(path)?
        .find_reference("refs/remotes/origin/HEAD")?
        .symbolic_target()
        .unwrap_or_default()
        .split('/')
        .last()
        .unwrap_or_default()
        .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_finds_default_branch() {
        let default_branch =
            default_branch("./tests/integration/").expect("Default branch not found");

        assert_eq!(default_branch, "main");
    }
}
