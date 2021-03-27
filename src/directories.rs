use crate::errors;
use crate::tree;
use errors::{AppResult, Error};
use glob::glob;
use std::{fs, path::PathBuf};
use tree::directory_tree;

pub fn prep_tmp_dir() -> AppResult<String> {
    let tmp_dir = format!(
        "{}.{}",
        std::env::temp_dir()
            .to_str()
            .expect("Couldn't locate temp directory"),
        env!("CARGO_PKG_NAME")
    );
    if std::path::Path::new(&tmp_dir).is_dir() {
        fs::remove_dir_all(&tmp_dir)?;
    }
    fs::create_dir(&tmp_dir)?;

    Ok(tmp_dir)
}

pub fn check_distination(destination: &str, force: bool) -> AppResult<String> {
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

pub fn move_to_destination(
    tmp_dir: &str,
    destination: &str,
    filter: Option<String>,
    preview: bool,
) -> AppResult<()> {
    let full_filter = match filter {
        Some(f) => format!("{}/{}", &tmp_dir, &f),
        None => format!("{}/{}", &tmp_dir, "*"),
    };

    let matches = glob(&full_filter).expect("Failed to read glob pattern");
    if matches.count() == usize::MIN {
        return Err(Error::NoMatchingFiles);
    }

    let destination_path = PathBuf::from(&destination);

    if preview {
        println!("🔬 The following would be copied to {}.\n", destination);
    } else {
        if !destination_path.exists() {
            fs::create_dir(&destination_path)?;
        }
    }

    for entry in glob(&full_filter).expect("Failed to read glob pattern") {
        if preview {
            directory_tree(entry.expect("Error traversing directory."))?;
        } else {
            match entry {
                Ok(path) => {
                    let source_file = path.display().to_string();
                    let file_name = path
                        .file_name()
                        .expect("File should have a name")
                        .to_owned()
                        .to_str()
                        .unwrap_or_default()
                        .to_owned();
                    let dest_file = format!("{}/{}", &destination, &file_name);

                    // println!("Copy from {} to {}", &source_file, &dest_file);

                    fs::rename(&source_file, &dest_file)?;
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    Ok(())
}
