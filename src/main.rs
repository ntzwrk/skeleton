#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate clap;
extern crate hyper;
extern crate hyper_native_tls;

mod gitignore;

use std::fs::{File, create_dir};
use std::io::{BufReader, Result, Error, ErrorKind};
use std::io::prelude::*;
use std::process::exit;

use clap::{App, SubCommand};

#[derive(Deserialize)]
struct Config {
    mkdir: Option<Vec<String>>,
    touch: Option<Vec<String>>,
    exec: Option<Vec<String>>,
}

fn main() {
    let res = gitignore::get_gitignore(vec!["rust".to_string(), "vim".to_string()]).unwrap();
    println!("{}", res);
    exit(1);
    let matches = App::new("test")
        .version("0.1")
        .author("Valentin B. <mail@mail.mail>")
        .about("Project manager")
        .args_from_usage("-l, --lang=[LANG]  'Set language configuration'")
        .subcommand(SubCommand::with_name("new")
                        .about("create new project")
                        .arg_from_usage("<NAME>   'The project name'"))
        .subcommand(SubCommand::with_name("init").about("initialize existing project"))
        .get_matches();
    let lang = matches.value_of("lang").unwrap().to_string();

    if let Some(matches) = matches.subcommand_matches("new") {
        println!("doing new {}", matches.value_of("NAME").unwrap());
    }

    let config: Config = match parse_config(lang) {
        Ok(c) => c,
        Err(err) => {
            println!("Error while parsing config! {:?}", err);
            exit(1);
        }
    };
    // println!("mkdir: {}", (config.mkdir.unwrap())[0]);
    for dir in config.mkdir.unwrap() {
        create_dir(dir);
    }
    println!("touch: {}", (config.touch.unwrap())[0]);
    println!("exec: {}", (config.exec.unwrap())[0]);
}

fn parse_config(file_name: String) -> Result<Config> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader.read_to_string(&mut contents)?;

    match toml::from_str(&contents) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::new(ErrorKind::InvalidInput, err)),
    }
}
