use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

fn curr_ms() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string()
}

#[test]
fn no_repo_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rsieve")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("The following required arguments"));

    Ok(())
}

#[test]
fn it_helps() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rsieve")?;
    cmd.arg("-h");

    cmd.assert().success().stdout(predicate::str::contains(
        "[FLAGS] [OPTIONS] <repo> [destination]",
    ));

    Ok(())
}

#[test]
fn it_versions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rsieve")?;
    cmd.arg("-V");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rsieve"));

    Ok(())
}

#[test]
fn it_previews() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rsieve")?;
    cmd.arg("-p");
    cmd.arg("bradyjoslin/sharewifi");
    cmd.arg("asdfghjk");

    cmd.assert().success().stdout(predicate::str::contains(
        "🔬 These files would be copied to",
    ));

    Ok(())
}

#[test]
fn it_gets_tarball() -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs, path::PathBuf};

    let dir = &format!("{}-{}", "it_gets_tarball", curr_ms());

    let mut cmd = Command::cargo_bin("rsieve")?;
    cmd.arg("bradyjoslin/sharewifi");
    cmd.arg(dir);

    cmd.assert().success();

    let path = PathBuf::from(dir);

    assert_eq!(path.exists(), true);

    if path.exists() {
        let dir = fs::read_dir(&path).expect("should be able to read existing dir");
        let count = dir.count();
        let contains_files = count > 0;

        assert_eq!(contains_files, true);
    }

    Ok(())
}

#[test]
fn it_filters_tarball() -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs, path::PathBuf};

    let dir = &format!("{}-{}", "it_filters_tarball", curr_ms());

    let mut cmd = Command::cargo_bin("rsieve")?;
    cmd.args(&["--filter", "LICENSE"]);
    cmd.arg("bradyjoslin/sharewifi");
    cmd.arg(dir);

    cmd.assert().success();

    let path = PathBuf::from(dir);
    assert_eq!(path.exists(), true);

    if path.exists() {
        let dir = fs::read_dir(&path).expect("should be able to read existing dir");
        let count = dir.count();
        let contains_a_file = count == 1;

        assert_eq!(contains_a_file, true);
    }

    Ok(())
}
