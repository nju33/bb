// use std::convert::AsRef;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Bookmark {
  pub name: String,
  path: PathBuf,
}

impl Bookmark {
  pub fn new(name: &str, path: &str) -> Bookmark {
    Bookmark {
      name: name.into(),
      path: Path::new(path).to_path_buf(),
    }
  }

  pub fn command(&self, head: PathBuf) -> String {
    let path = &head.join(self.path.to_str().unwrap());
    format!("{}", path.to_str().unwrap())
  }
}