use crate::errors;
use errors::AppResult;
use ptree::{Style, TreeItem};
use std::borrow::Cow;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Clone, Debug)]
pub struct PathItem(pub PathBuf);

impl TreeItem for PathItem {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        if let Some(n) = self.0.file_name() {
            write!(f, "{}", style.paint(n.to_string_lossy()))
        } else {
            Ok(())
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        let v = if let Ok(list) = fs::read_dir(&self.0) {
            list.filter_map(|item| item.ok())
                .map(|entry| entry.path())
                .map(PathItem)
                .collect()
        } else {
            Vec::new()
        };

        Cow::from(v)
    }
}

pub fn directory_tree(dir: PathBuf) -> AppResult<()> {
    ptree::print_tree(&PathItem(dir))?;

    Ok(())
}

#[test]
#[ignore]
fn it_parses_repo_stem_input() {
    let mut dir = PathBuf::new();
    dir.push(std::env::current_dir().expect("Unable to get current directory"));
    let res = directory_tree(dir);

    assert_eq! {res.is_ok() , true};
}
