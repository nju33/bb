#[macro_use]
extern crate serde_derive;
extern crate clap;

pub mod bookmark;
mod config;

use std::env;
use std::io::{Error, ErrorKind};
use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use std::fs::File;
use std::path::{Path, PathBuf};
use clap::{Arg, App, SubCommand};
use config::{Config, ConfigErr};
use bookmark::Bookmark;

fn run_app() -> Result<(), ConfigErr> {
    let config = Config::new();
    try!(config.get());

    let mut args = env::args();
    let target_name = args.nth(1).unwrap();
    // let target_name = args.nth(3).unwrap();

    let bookmarks = config.bookmarks.borrow();
    let target_bookmark = bookmarks.iter().find(|bookmark| {
        bookmark.name == target_name
    });

    match target_bookmark {
        Some(b) => {
            let dir_name = config.dir_name.take();
            println!("{}", b.command(dir_name));
            Ok(())
        },
        None => Err(ConfigErr::Io(Error::new(ErrorKind::NotFound, "not found"))),
    }
}

fn main() -> () {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            match err {
                ConfigErr::Io(err) => {
                    eprintln!("{:?}", err)
                }
                _ => {
                    eprintln!("{:?}", "err")
                }
            };
            1
        }
    });
}