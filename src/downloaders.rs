use crate::errors;
use errors::{AppResult, Error};
use flate2::read::GzDecoder;
use run_script::ScriptOptions;
use std::fs;
use tar::Archive;

pub fn git_clone(repo: &str, dir: &str, branch: Option<String>) -> AppResult<()> {
    let repo_url = format!("git@github.com:{}.git", repo);
    get_with_git(&repo_url, &dir, branch)?;
    fs::remove_dir_all(format!("{}/.git", &dir))?;

    Ok(())
}

pub async fn get_tarball(repo: &str, dir: &str, branch: Option<String>) -> AppResult<()> {
    let stem_branch = if branch.is_some() {
        format!("archive/refs/heads/{}.tar.gz", branch.unwrap_or_default())
    } else {
        "archive/HEAD.tar.gz".into()
    };

    let repo_url = format!("https://github.com/{}/{}", repo, stem_branch);

    let archive = download(&repo_url).await?;
    unzip(&dir, &archive)?;

    Ok(())
}

async fn download(url: &str) -> AppResult<Vec<u8>> {
    let client = reqwest::Client::new();

    let res_raw = client
        .get(url)
        .header(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(env!("CARGO_PKG_NAME")),
        )
        .send()
        .await?;

    if res_raw.status().is_client_error() {
        return Err(Error::ClientWithStatus(res_raw.status()));
    }
    let res_bytes = res_raw.bytes().await?;
    let res_slice = res_bytes.as_ref().to_owned();

    Ok(res_slice)
}

fn get_with_git(url: &str, dest: &str, branch: Option<String>) -> AppResult<()> {
    let options = ScriptOptions::new();

    if branch.is_some() {
        let (code, _, err) = run_script::run_script!(
            r#"git clone -b "$1" --depth 1 "$2" "$3""#,
            &vec![branch.unwrap_or_default(), url.into(), dest.into()],
            options
        )
        .expect("Couldn't run script");
        if code > 0 {
            return Err(Error::CloneError(err));
        }
    } else {
        let (code, _, err) = run_script::run_script!(
            r#"git clone --depth 1 "$1" "$2""#,
            &vec![url.into(), dest.into()],
            options
        )
        .expect("Couldn't run script");
        if code > 0 {
            return Err(Error::CloneError(err));
        }
    }

    Ok(())
}

fn unzip(dest: &str, res: &[u8]) -> AppResult<()> {
    let tar = GzDecoder::new(res);
    let mut archive = Archive::new(tar);
    let files = archive.entries()?.enumerate();

    for (_, file) in files {
        let mut file = file.unwrap();
        let file_path = file.path()?;

        // trim off the root directory in the archive
        let mut components = file_path.components();
        let root_dir = components.next().unwrap();
        let new_path = file_path
            .strip_prefix(root_dir)?
            .to_str()
            .unwrap()
            .to_owned();

        file.unpack(format!("{}/{}", dest, new_path)).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn curr_ms() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            .to_string()
    }

    #[test]
    #[ignore]
    fn it_gets_github_git_repos() {
        use std::{fs, path::PathBuf};

        let repo = "bradyjoslin/sharewifi";
        let dir = &format!("{}-{}", "it_gets_github_git_repos", curr_ms());
        let res = git_clone(repo, dir, None);

        assert_eq!(res.is_ok(), true);

        let path = PathBuf::from(dir);

        assert_eq!(path.exists(), true);

        if path.exists() {
            let dir = fs::read_dir(&path).expect("should be able to read existing dir");
            let count = dir.count();
            let contains_files = count > 0;

            assert_eq!(contains_files, true);
        }
    }

    #[tokio::test]
    async fn it_gets_github_tarball_repos() {
        use std::{fs, path::PathBuf};

        let repo = "bradyjoslin/sharewifi";
        let dir = &format!("{}-{}", "it_gets_github_tarball_repos", curr_ms());
        let res = get_tarball(repo, dir, None).await;

        assert_eq!(res.is_ok(), true);

        let path = PathBuf::from(dir);

        assert_eq!(path.exists(), true);

        if path.exists() {
            let dir = fs::read_dir(&path).expect("should be able to read existing dir");
            let count = dir.count();
            let contains_files = count > 0;

            assert_eq!(contains_files, true);
        }
    }

    #[tokio::test]
    async fn it_fails_nonexist_github_tarball_repos() {
        let repo = "bradyjoslin/sharewifisss";
        let dir = "it_fails_nonexist_github_tarball_repos";
        let res = get_tarball(repo, dir, None).await;

        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn it_fails_nonexist_github_git_repos() {
        let repo = "bradyjoslin/sharewifisss";
        let dir = "it_fails_nonexist_github_git_repos";
        let res = git_clone(repo, dir, None);

        assert_eq!(res.is_err(), true);
    }
}
