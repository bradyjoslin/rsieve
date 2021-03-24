// use app::App;
use flate2::read::GzDecoder;
use run_script::ScriptOptions;
use std::path::PathBuf;
use structopt::StructOpt;
use tar::Archive;

mod app;
mod errors;

use errors::{AppResult, Error};

#[tokio::main]
async fn main() -> AppResult<()> {
    let app = app::App::from_args();
    let repo = &app.repo;
    let destination = check_input(&app.destination)?;
    let git = &app.git;

    if !check_distination(&destination).unwrap() {
        panic!("bad directory!")
    }

    if git.clone() {
        let repo_url = build_url(repo, "git").unwrap();
        get_with_git(&repo_url, &destination);
        std::fs::remove_dir_all(format!("{}/.git", &destination)).unwrap();
    } else {
        let repo_url = build_url(repo, "tar").unwrap();
        let archive = download(&repo_url).await.unwrap();
        unzip(&destination, &archive).unwrap();
    }

    Ok(())
}

fn check_distination(destination: &str) -> AppResult<bool> {
    let path = PathBuf::from(destination);
    if path.exists() {
        let dir = std::fs::read_dir(&path);
        let count = dir.unwrap().count();

        if count != 0 {
            return Ok(false);
        }
    }

    Ok(true)
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
        .await
        .unwrap();
    let res_bytes = res_raw.bytes().await.unwrap();
    let res_slice = res_bytes.as_ref().to_owned();

    Ok(res_slice)
}

fn unzip(dest: &str, res: &[u8]) -> AppResult<()> {
    let tar = GzDecoder::new(res);
    let mut archive = Archive::new(tar);
    let files = archive.entries().unwrap().enumerate();

    for (_, file) in files {
        let mut file = file.unwrap();
        let file_path = file.path().unwrap();

        // trim off the root directory in the archive
        let root_dir = file_path.components().next().unwrap();
        let new_path = file_path
            .strip_prefix(root_dir)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        file.unpack(format!("{}/{}", dest, new_path)).unwrap();
    }

    Ok(())
}

fn check_input(input: &str) -> AppResult<String> {
    if input == "BadInput" {
        Err(Error::BadInput)
    } else {
        Ok(input.to_owned())
    }
}
