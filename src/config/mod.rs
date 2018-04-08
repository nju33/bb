extern crate toml;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Error};
// use std::iter::IntoIterator::into_iter;
use std::collections::{BTreeMap};
use self::toml::Value;
use bookmark::Bookmark;

#[derive(Deserialize, Debug)]
pub struct Config {
  aliases: BTreeMap<String, Value>
}

pub fn hello_mod() {
  println!("{}", "from mod");
}

impl Config {
  fn find_bookmark_toml() -> Option<String> {
    let mut current_dir = env::current_dir().unwrap();
    let mut current_dir = Some(Path::new(current_dir.to_str().unwrap()));
    let mut filename: Option<String> = None;

     while let Some(dirname) = current_dir {
      filename = Path::new(&dirname).with_file_name("bookmark.toml").to_str().map(|str| str.to_string());
      if Path::new(&filename.unwrap()).exists() {
        filename = filename;
        break;
      }

      current_dir = Path::new(&dirname).parent();
     }

     filename.and_then(|name| name.into())
  }

  pub fn get() -> Result<String, Error> {
    let current_dir = env::current_dir().unwrap();;
    println!("{:?}", current_dir);
    let filename = Path::new(&current_dir).with_file_name("a.toml");
    let mut file = File::open(&filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    // let parsed = contents.parse::<Value>().unwrap();
    // let a = &parsed["aliases"];
    // let a: Vec<(String, String)> = parsed.try_into().unwrap();
    // println!("{:?}", a);
    // for (movie, review) in &parsed.iter() {
      // println!("{}: \"{}\"", movie, review);
    // }

    let config: Config = toml::from_str(&contents).unwrap();
    // println!("{:?}", aliases);



    // a.into_iter().map(|b| println!("{:?}", b));
    // for (name, path) in &config.aliases {
    //   println!("{}/{}", name, path);
    // }

    let a: Vec<_> = config.aliases.into_iter().map(|a| {
      match a {
        (n, p) => Bookmark::new(n, p.to_string()),
        _ => panic!(),
      }
      // println!("{:?}", a);
      // a
    }).collect();
    println!("{:?}", a);

    // parsed.iter().map(|a| println!("{:?}", a));
    // println!("{:?}", parsed["hoge"]);

    Ok("aiueo".to_string())
  }
}