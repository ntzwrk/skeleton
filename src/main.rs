#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate clap;
extern crate hyper;
extern crate hyper_native_tls;

mod gitignore;

use std::fs::{File, OpenOptions, create_dir, read_dir};
use std::io::{BufReader, Result, Error, ErrorKind};
use std::io::prelude::*;
use std::path::Path;
use std::process::{exit, Command};
use std::env;

use clap::{App, SubCommand};

#[derive(Deserialize)]
struct Config {
    mkdir: Option<Vec<String>>,
    touch: Option<Vec<String>>,
    exec: Option<Vec<String>>,
    gitignore: Option<Vec<String>>,
    include: Option<Vec<String>>,
}

fn main() {
    let matches = App::new("Skeleton")
        .version("0.1")
        .author("Valentin B. <mail@mail.mail>")
        .about("Skeleton project manager")
        .args_from_usage("-l, --lang=<LANG>  'Set language configuration'")
        .subcommand(SubCommand::with_name("new")
                        .about("create new project")
                        .arg_from_usage("<NAME>   'The project name'"))
        .subcommand(SubCommand::with_name("init").about("initialize existing project"))
        .get_matches();
    let lang = matches.value_of("lang").unwrap();

    if let Some(matches) = matches.subcommand_matches("new") {
        println!("doing new {}", matches.value_of("NAME").unwrap());
    }

    let mut config_path = match env::home_dir() {
        Some(path) => {
            match path.to_str() {
                Some(p) => p.to_string(),
                None => {
                    println!("Could not resolve $HOME");
                    exit(1);
                }
            }
        }
        None => {
            println!("Could not resolve $HOME");
            exit(1);
        }
    };

    config_path.push_str("/.skeleton/");
    config_path.push_str(lang);
    config_path.push_str(".toml");

    let config: Config = match parse_config(config_path) {
        Ok(c) => c,
        Err(err) => {
            println!("Error while parsing config! {:?}", err);
            exit(1);
        }
    };

    if let Some(subcmd) = matches.subcommand_matches("new") {
        let name = subcmd.value_of("NAME").unwrap();
        if file_exists(&name.to_string()) {
            println!("A file or directory with that name already exists");
            exit(1);
        } else {
            match create_dir(name.clone()) {
                Ok(_) => println!("Created directory '{}'", name),
                Err(e) => {
                    println!("Could not create direcotry '{}'. {:?}", name, e);
                    exit(1);
                }
            }
            if let Err(_) = env::set_current_dir(Path::new(&*name)) {
                println!("Could not change directory!");
                exit(1);
            }
        }
    }

    if !dir_is_empty(&".".to_string()) {
        println!("Direcotry is not empty!");
        exit(1);
    }

    if let Some(dirs) = config.mkdir {
        for dir in dirs {
            if !file_exists(&dir) {
                match create_dir(dir.clone()) {
                    Ok(_) => println!("Created directory '{}'", dir),
                    Err(e) => println!("Could not create directory '{}'. {:?}", dir, e),
                }
            } else {
                println!("Directory '{}' already exists", dir);
            }
        }
    }

    if let Some(gi) = config.gitignore {
        let ign = gitignore::get_gitignore(gi).unwrap();
        let file = OpenOptions::new()
            .create_new(true)
            .append(true)
            .open("gitignore");
        if let Ok(mut file) = file {
            writeln!(file, "{}", ign);
        } else {
            println!("Could not write .gitignore");
        }
    }

    if let Some(fnames) = config.touch {
        for file in fnames {
            if !create_file(&file) {
                println!("Could not create file '{}'", file);
            }
        }
    }

    if let Some(cmds) = config.exec {
        for cmd in cmds {
            println!("Executing '{}'", cmd);
            match exec(&cmd) {
                Ok((out, err)) => {
                    println!("stdout: {}", out);
                    println!("stderr: {}", err);
                }
                Err(e) => println!("Failed to execute: {:?}", e),
            }
        }
    }
}

fn dir_is_empty(dir: &String) -> bool {
    let paths = read_dir(dir).unwrap();

    for _ in paths {
        return false;
    }
    true
}

fn file_exists(name: &String) -> bool {
    Path::new(&name).exists()
}

fn exec(cmd: &String) -> Result<(String, String)> {
    #[cfg(unix)]
    let shell = "sh";
    #[cfg(unix)]
    let arg1 = "-c";
    #[cfg(windows)]
    let shell = "cmd.exe";
    #[cfg(windows)]
    let arg1 = "/C";

    let out = match Command::new(shell).arg(arg1).arg(cmd).output() {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let stdout = match String::from_utf8(out.stdout) {
        Ok(so) => so,
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
    };

    let stderr = match String::from_utf8(out.stderr) {
        Ok(se) => se,
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
    };

    Ok((stdout, stderr))
}

fn create_file(file_name: &String) -> bool {
    !file_exists(&file_name) &&
    match OpenOptions::new()
              .append(true)
              .create(true)
              .open(file_name) {
        Ok(_) => true,
        Err(_) => false,
    }
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

#[test]
fn test_file_exists() {
    assert!(file_exists(&"test/foo.toml".to_string()));
    assert!(!file_exists(&"test/none".to_string()));
}

#[test]
fn test_parse_config() {
    let conf = parse_config("test/foo.toml".to_string()).unwrap();
    let gi = conf.gitignore.unwrap();
    assert_eq!(gi, vec!["rust".to_string(), "vim".to_string()]);

    let mkd = conf.mkdir.unwrap();
    assert_eq!(mkd, vec!["b".to_string()]);

    let touch = conf.touch.unwrap();
    assert_eq!(touch, vec!["a".to_string()]);

    let exec = conf.exec.unwrap();
    assert_eq!(exec, vec!["touch asdf".to_string()]);
}

#[test]
fn test_exec() {
    let (stdout, stderr) = exec(&"echo -n test".to_string()).unwrap();
    assert_eq!(stdout, "test".to_string());
    assert_eq!(stderr, "".to_string());
    let (stdout, stderr) = exec(&"echo -n test 1>&2".to_string()).unwrap();
    assert_eq!(stdout, "".to_string());
    assert_eq!(stderr, "test".to_string());
}

#[test]
fn test_dir_is_empty() {
    let is_empty = dir_is_empty(&"./test".to_string());
    assert!(!is_empty);
}
