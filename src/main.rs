use flate2::read::GzDecoder;
use run_script::ScriptOptions;
use std::{fs, path::PathBuf};
use structopt::StructOpt;
use tar::Archive;

mod app;
mod errors;

use errors::{AppResult, Error};

#[tokio::main]
async fn main() -> AppResult<()> {
    let app = app::App::from_args();
    let repo = app.repo;
    let force = app.force;
    let destination = check_distination(&app.destination, force)?;
    let git = app.git;
    let workflows = app.workflows;
    let tmp_dir = format!(
        "{}.{}",
        std::env::temp_dir().to_str().unwrap_or("tmp/"),
        env!("CARGO_PKG_NAME")
    );

    prep_tmp_dir(&tmp_dir)?;

    if git {
        git_clone(&repo, &tmp_dir)?;
    } else {
        get_tarball(&repo, &tmp_dir).await?;
    }

    if workflows {
        workflows_only(&tmp_dir)?;
    }

    move_to_destination(&tmp_dir, &destination)?;

    Ok(())
}

fn prep_tmp_dir(tmp_dir: &str) -> AppResult<()> {
    if std::path::Path::new(&tmp_dir).is_dir() {
        fs::remove_dir_all(&tmp_dir)?;
    }
    fs::create_dir(&tmp_dir)?;

    Ok(())
}

fn git_clone(repo: &str, tmp_dir: &str) -> AppResult<()> {
    let repo_url = build_url(&repo, "git")?;
    get_with_git(&repo_url, &tmp_dir);
    fs::remove_dir_all(format!("{}/.git", &tmp_dir))?;

    Ok(())
}

async fn get_tarball(repo: &str, tmp_dir: &str) -> AppResult<()> {
    let repo_url = build_url(&repo, "tar")?;
    let archive = download(&repo_url).await?;
    unzip(&tmp_dir, &archive)?;

    Ok(())
}

fn workflows_only(tmp_dir: &str) -> AppResult<()> {
    let path = PathBuf::from(tmp_dir);
    if path.exists() {
        let dir = fs::read_dir(&path)?;
        for entry in dir {
            let file = &entry?;
            let file_name = &file
                .file_name()
                .to_owned()
                .to_str()
                .unwrap_or_default()
                .to_owned();

            let file_type = file.file_type()?;
            let file_path = format!("{}/{}", &tmp_dir, &file_name);

            if file_name != ".github" {
                if file_type.is_dir() {
                    fs::remove_dir_all(file_path)?;
                } else {
                    fs::remove_file(file_path)?;
                }
            }
        }
    } else {
        // TODO: return an error instead.
        println!("path doesn't exist?");
    }

    Ok(())
}

fn check_distination(destination: &str, force: bool) -> AppResult<String> {
    let path = PathBuf::from(destination);
    if path.exists() {
        let dir = fs::read_dir(&path)?;
        let count = dir.count();

        if count != 0 && !force {
            // println!("Count: {}", count);
            return Err(Error::DesinationNotEmpty);
        }
    }

    Ok(destination.into())
}

fn move_to_destination(tmp_dir: &str, destination: &str) -> AppResult<()> {
    let temp_path = PathBuf::from(tmp_dir);
    let destination_path = PathBuf::from(destination);

    if !destination_path.exists() {
        fs::create_dir(destination_path)?;
    }

    if temp_path.exists() {
        let dir = fs::read_dir(&temp_path)?;
        for entry in dir {
            let file = &entry?;
            let file_name = &file
                .file_name()
                .to_owned()
                .to_str()
                .unwrap_or_default()
                .to_owned();

            let source_file = format!("{}/{}", &tmp_dir, &file_name);
            let dest_file = format!("{}/{}", &destination, &file_name);
            // println!("Copy from {} to {}", &source_file, &dest_file);

            fs::rename(&source_file, &dest_file)?;
        }
    } else {
        // TODO: return an error instead.
        println!("path doesn't exist?");
    }

    Ok(())
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
        .await?;
    let res_bytes = res_raw.bytes().await?;
    let res_slice = res_bytes.as_ref().to_owned();

    Ok(res_slice)
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

        // if workflows {
        //     let x = components.next();
        //     if x.is_some() && x == Some(Component::Normal(OsStr::new(".github"))) {
        //         file.unpack(format!("{}/{}", dest, new_path)).unwrap();
        //     }
        // } else {
        file.unpack(format!("{}/{}", dest, new_path)).unwrap();
        // }
    }

    Ok(())
}
