use crate::errors;
use errors::{AppResult, Error};
use flate2::read::GzDecoder;
use run_script::ScriptOptions;
use std::fs;
use tar::Archive;

pub fn git_clone(repo: &str, dir: &str) -> AppResult<()> {
    let repo_url = build_url(&repo, "git")?;
    get_with_git(&repo_url, &dir);
    fs::remove_dir_all(format!("{}/.git", &dir))?;

    Ok(())
}

pub async fn get_tarball(repo: &str, dir: &str) -> AppResult<()> {
    let repo_url = build_url(&repo, "tar")?;
    let archive = download(&repo_url).await?;
    unzip(&dir, &archive)?;

    Ok(())
}

fn build_url(repo: &str, mode: &str) -> AppResult<String> {
    match mode {
        "tar" => return Ok(format!("https://github.com/{}/archive/HEAD.tar.gz", repo)),
        "git" => return Ok(format!("git@github.com:{}.git", repo)),
        _ => return Err(Error::BadInput),
    };
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

fn get_with_git(url: &str, dest: &str) {
    let options = ScriptOptions::new();

    let (_, _, _) = run_script::run_script!(
        r#"git clone --depth 1 $1 $2"#,
        &vec![url.into(), dest.into()],
        options
    )
    .expect("Couldn't git it");
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
