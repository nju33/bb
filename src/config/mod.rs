extern crate toml;

use self::toml::{Value, de};
use std::fmt;
use std::error;
use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::cell::{Cell, RefCell};
use std::io::{Read, Error, ErrorKind};
use std::collections::{BTreeMap};
use bookmark::Bookmark;

pub struct Config {
  pub dir_name: Cell<PathBuf>,
  pub bookmarks: RefCell<Vec<Bookmark>>,
}

impl Config {
  pub fn new() -> Config {
    Config {
      dir_name: Cell::new(PathBuf::new()),
      bookmarks: RefCell::new(Vec::new()),
    }
  }

  fn get_file_name<T: AsRef<Path>>(&self, dir_name: T) -> PathBuf {
    dir_name.as_ref().join("bookmark.toml")
  }

  fn find_bookmark_toml(&self) -> Result<PathBuf, ConfigErr> {
    let mut current_dir_name = Path::new("/Users/nju33/github/bb/example/b").to_path_buf();
    let mut dir_name = current_dir_name.clone();
    let mut file_name = self.get_file_name(&dir_name);  

    while !file_name.exists() {
      try!(match dir_name.clone().parent() {
        Some(parent_dir_name) => {
          current_dir_name = parent_dir_name.to_path_buf();
          dir_name = current_dir_name;
          file_name = self.get_file_name(&parent_dir_name);
          Ok("ok")
        },
        None => {
          Err(ConfigErr::Io(Error::new(ErrorKind::NotFound, "bookmark.toml not found")))
        },
      });
    }

    Ok(file_name)
  }

  pub fn get(&self) -> ::std::result::Result<(), ConfigErr> {
    let bookmark_toml: PathBuf = try!(self.find_bookmark_toml());
    let mut file = try!(File::open(&bookmark_toml).map_err(ConfigErr::Io));
    let mut contents = String::new();
    file.read_to_string(&mut contents);
  
    let toml = try!(contents.parse::<Value>());
    let bookmark_table = toml.as_table().unwrap();

    let bookmarks = bookmark_table.into_iter().map(|cell| {
      Bookmark::new(cell.0.as_str(), cell.1.as_str().unwrap())
    }).collect::<Vec<Bookmark>>();

    self.dir_name.replace(bookmark_toml.parent().unwrap().to_path_buf());
    self.bookmarks.replace(bookmarks);

    Ok(())
  }
}

#[derive(Debug)]
pub enum ConfigErr {
  Io(::std::io::Error),
  Toml(de::Error),
}

impl fmt::Display for ConfigErr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ConfigErr::Io(ref err) => write!(f, "IO Error: {}", err),
      ConfigErr::Toml(ref err) => write!(f, "Toml Error: {}", err),
    }
  }
}

impl error::Error for ConfigErr {
  fn description(&self) -> &str {
    match *self {
      ConfigErr::Io(ref err) => err.description(),
      ConfigErr::Toml(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      ConfigErr::Io(ref err) => Some(err),
      ConfigErr::Toml(ref err) => Some(err),
    }
  }
}

impl From<::std::io::Error> for ConfigErr {
  fn from(err: ::std::io::Error) -> ConfigErr {
    ConfigErr::Io(err)
  }
}

impl From<de::Error> for ConfigErr {
  fn from(err: de::Error) -> ConfigErr {
    ConfigErr::Toml(err)
  }
}