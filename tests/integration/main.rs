use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

fn binary() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

fn tmpdir(test_name: &str) -> String {
    let base_test_dir = format!(
        "{}/.{}-test",
        home::home_dir()
            .expect("Couldn't locate home directory")
            .display(),
        env!("CARGO_PKG_NAME")
    );

    if !std::path::Path::new(&base_test_dir).is_dir() {
        std::fs::create_dir(&base_test_dir).expect("Create test dir");
    }

    format!("{}/{}-{}", base_test_dir, test_name, curr_ms())
}

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
    binary()
        .assert()
        .failure()
        .stderr(predicate::str::contains("The following required arguments"));

    Ok(())
}

#[test]
fn it_helps() -> Result<(), Box<dyn std::error::Error>> {
    binary()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "[FLAGS] [OPTIONS] <repo> [destination]",
        ));

    Ok(())
}

#[test]
fn it_versions() -> Result<(), Box<dyn std::error::Error>> {
    binary()
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("rsieve"));

    Ok(())
}

#[test]
fn it_previews() -> Result<(), Box<dyn std::error::Error>> {
    binary()
        .arg("-p")
        .arg("bradyjoslin/sharewifi")
        .arg("asdfghjk")
        .assert()
        .success()
        .stdout(predicate::str::contains("These files from"));

    Ok(())
}

#[test]
fn it_gets_tarball() -> Result<(), Box<dyn std::error::Error>> {
    use std::{fs, path::PathBuf};

    let dir = tmpdir("it_gets_tarball");

    binary()
        .arg("bradyjoslin/sharewifi")
        .arg(&dir)
        .assert()
        .success();

    let path = PathBuf::from(&dir);

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

    let dir = tmpdir("it_filters_tarball");

    binary()
        .args(&["--filter", "LICENSE"])
        .arg("bradyjoslin/sharewifi")
        .arg(&dir)
        .assert()
        .success();

    let path = PathBuf::from(&dir);
    assert_eq!(path.exists(), true);

    if path.exists() {
        let dir = fs::read_dir(&path).expect("should be able to read existing dir");
        let count = dir.count();
        let contains_a_file = count == 1;

        assert_eq!(contains_a_file, true);
    }

    Ok(())
}
