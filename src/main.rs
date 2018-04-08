#[macro_use]
extern crate serde_derive;
extern crate clap;

pub mod bookmark;
mod config;

use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use std::fs::File;
use std::path::Path;
use clap::{Arg, App, SubCommand};
use config::Config;
// use std::io::{Command};


fn main() -> () {
    let app = App::new("bb")
                        .version("1.0")
                        .author("nju33 <nju33.ki@gmail.com>")
                        .about("");
//                         .arg(Arg::with_name("config")
//                             .short("c")
//                             .long("config")
//                             .value_name("FILE")
//                             .help("Sets a custom config file")
//                             .takes_value(true))
//                         .arg(Arg::with_name("INPUT")
//                             .help("Sets the input file to use")
//                             .required(true)
//                             .index(1))
//                         .arg(Arg::with_name("v")
//                             .short("v")
//                             .multiple(true)
//                             .help("Sets the level of verbosity"))
//                         .subcommand(SubCommand::with_name("test")
//                                     .about("controls testing features")
//                                     .version("1.3")
//                                     .author("Someone E. <someone_else@other.com>")
//                                     .arg(Arg::with_name("debug")
//                                         .short("d")
//                                         .help("print debug information verbosely")))
//                         .get_matches();
    // let child = Command::new("cd").args(vec!["a"]).exec()

    
    let current_dir = std::env::current_dir().unwrap().join("a");
    let root = Path::new(&current_dir);
    let result = std::env::set_current_dir(&root).is_ok();
    // println!("Successfully changed working directory to {}!", root.display());
    // println!("Hello, world!");
    // println!("{:?}", result);
    let config = Config::get();
    match config {
        Ok(ref result) => println!("{:?}", result),
        Err(ref e) => println!("{:?}", e)
    };


    println!("{:?}", format!("{} {:?}", "cd", &current_dir));
    // println!("{:?}", child);
    // match child {
    //     Ok(mut child) => {
    //         let a = child.wait().unwrap().code().unwrap();
    //         println!("{:?}", a);
    //         // 1
    //     }
    //     Err(err) => {
    //         eprintln!("{}", err.to_string());
    //         // 127
    //     }
    // }
}