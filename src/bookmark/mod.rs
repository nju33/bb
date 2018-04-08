use std::convert::AsRef;
use std::path::Path;

#[derive(Debug)]
pub struct Bookmark {
  name: String,
  path: String,
}

impl Bookmark {
  pub fn new(name: String, path: String) -> Bookmark {
    Bookmark {
      name: name,
      path: path,
    }
  }
}