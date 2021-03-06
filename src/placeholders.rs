use crate::errors;
use errors::AppResult;
use std::fs::{read_to_string, OpenOptions};
use std::io::prelude::*;

pub fn update_placeholder_branch(file_name: &str, default_branch: &str) -> AppResult<()> {
    let contents = read_to_string(file_name)?;
    let new = contents.replace("$default-branch", default_branch);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;
    file.write_all(new.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_replaces_branch_placeholder() {
        let file_name = "tests/sample_templates/android.yml";
        update_placeholder_branch(file_name, "main").expect("should be able to update test file");
        let after_contents = read_to_string(file_name).expect("test file not present");

        assert_eq!(after_contents.contains("$default-branch"), false);
        assert_eq!(after_contents.contains("main"), true);
    }
}
