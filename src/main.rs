#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate clap;
extern crate hyper;
extern crate hyper_native_tls;

mod gitignore;
mod cli;

use std::fs::{File, OpenOptions, create_dir, read_dir};
use std::io::{BufReader, Result, Error, ErrorKind};
use std::io::prelude::*;
use std::path::Path;
use std::process::{exit, Command};
use std::env;

#[derive(Deserialize)]
struct Config {
    mkdir: Option<Vec<String>>,
    touch: Option<Vec<String>>,
    exec: Option<Vec<String>>,
    gitignore: Option<Vec<String>>,
    include: Option<Vec<String>>,
    order: Option<Vec<String>>,
}

fn main() {
    let matches = cli::build_cli().get_matches();
    let lang = matches.value_of("lang").unwrap();

    if let Some(matches) = matches.subcommand_matches("new") {
        println!("doing new {}", matches.value_of("NAME").unwrap());
    }

    let config_path = match get_config_path(&lang.to_string()) {
        Some(p) => p,
        None => {
            println!("Could not find home directory");
            exit(1);
        }
    };

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
                    println!("Could not create directory '{}'. {:?}", name, e);
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
        println!("Directory is not empty!");
        exit(1);
    }

    if let Some(ref includes) = config.include {
        for incl in includes {
            let path = match get_config_path(&incl) {
                Some(p) => p,
                None => {
                    println!("Could not resolve home directory");
                    exit(1);
                }
            };

            let conf: Config = match parse_config(path) {
                Ok(c) => c,
                Err(e) => {
                    println!("Could not read include '{}'. {}", incl, e);
                    exit(1);
                }
            };
            process_config(conf);
        }
    }

    process_config(config);

}

fn mkdir(dirs: &Option<Vec<String>>) {
    if let Some(ref dirs) = *dirs {
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
}

fn gitignore(gi: &Option<Vec<String>>) {
    if let Some(ref gi) = *gi {
        let ign = gitignore::get_gitignore(&gi).unwrap();
        let file = OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(".gitignore");
        if let Ok(mut file) = file {
            match writeln!(file, "{}", ign) {
                Ok(_) => {}
                Err(e) => println!("Could not write .gitignore. {:?}", e),
            }
        } else {
            println!("Could not write .gitignore");
        }
    }
}

fn touch(files: &Option<Vec<String>>) {
    if let Some(ref fnames) = *files {
        for file in fnames {
            if !create_file(&file) {
                println!("Could not create file '{}'", file);
            }
        }
    }
}

fn exec_list(cmd_list: &Option<Vec<String>>) {
    if let Some(ref cmds) = *cmd_list {
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

fn process_config(config: Config) {
    let order: Vec<String> = match config.order {
        Some(order) => order,
        None => {
            vec!["mkdir".to_string(),
                 "gitignore".to_string(),
                 "touch".to_string(),
                 "exec".to_string()]
        }
    };

    for ord in order {
        match &*ord {
            "mkdir" => mkdir(&config.mkdir),
            "gitignore" => gitignore(&config.gitignore),
            "touch" => touch(&config.touch),
            "exec" => exec_list(&config.exec),
            x => println!("Unknown operation \"{}\"", x),
        }
    }
}

/// Returns the config path for a `lang` parameter.
///
/// This method concatenates $HOME with "/.skeleton/{lang}.toml"
///
/// # Examples
///
/// ```
/// let path = get_config_path(&"rust".to_string())
/// ```
fn get_config_path(lang: &String) -> Option<String> {
    let home = match env::home_dir() {
        Some(p) => p,
        None => return None,
    };
    let config_path_cow = home.to_string_lossy();
    let mut config_path = config_path_cow.into_owned();
    // let mut config_path = env!("HOME").to_string();

    config_path.push_str("/.skeleton/");
    config_path.push_str(lang);
    config_path.push_str(".toml");
    Some(config_path)
}

/// Checks if a directory is empty
///
/// # Examples
///
/// ```
/// let dir_is_empty(&".".to_string());
/// ```
fn dir_is_empty(dir: &String) -> bool {
    let paths = match read_dir(dir) {
        Ok(paths) => paths,
        Err(_) => return false,
    };

    for _ in paths {
        return false;
    }
    true
}

/// Checks if a file exists
fn file_exists(name: &String) -> bool {
    Path::new(&name).exists()
}

/// Executes a command using `sh` or `cmd.exe` depending on the used operating system.
///
/// Returns either a `Error` or a tuple `(stdout, stderr)`
fn exec(cmd: &String) -> Result<(String, String)> {
    #[cfg(unix)]
    let shell = "sh";
    #[cfg(unix)]
    let arg1 = "-c";
    #[cfg(windows)]
    let shell = "cmd.exe";
    #[cfg(windows)]
    let arg1 = "/C";

    let out = Command::new(shell).arg(arg1).arg(cmd).output()?;

    Ok((String::from_utf8(out.stdout).unwrap_or("Could not parse stdout".to_string()),
        String::from_utf8(out.stderr).unwrap_or("Could not parse stderr".to_string())))

}

/// Creates a file if it doesn't exist.
fn create_file(file_name: &String) -> bool {
    !file_exists(&file_name) &&
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .is_ok()
}

/// Parses a configuration file and returns the `Config` struct.
fn parse_config(file_name: String) -> Result<Config> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader.read_to_string(&mut contents)?;

    toml::from_str(&contents).map_err(|err| Error::new(ErrorKind::InvalidInput, err))
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
    let (stdout, stderr) = exec(&"echo test".to_string()).unwrap();
    #[cfg(unix)]
    assert_eq!(stdout, "test\n".to_string());
    #[cfg(windows)]
    assert_eq!(stdout, "test \r\n".to_string());
    assert_eq!(stderr, "".to_string());
    let (stdout, stderr) = exec(&"echo test 1>&2".to_string()).unwrap();
    assert_eq!(stdout, "".to_string());
    #[cfg(unix)]
    assert_eq!(stderr, "test\n".to_string());
    #[cfg(windows)]
    assert_eq!(stderr, "test \r\n".to_string());
}

#[test]
fn test_dir_is_empty() {
    let is_empty = dir_is_empty(&"./test".to_string());
    assert!(!is_empty);
    let non_existent = dir_is_empty(&"./non_existent_dir".to_string());
    assert!(!non_existent);
}

#[test]
fn test_get_config_path() {
    let expected = get_config_path(&"foo".to_string()).unwrap();
    let mut actual = env::home_dir().unwrap().to_str().unwrap().to_string();
    actual.push_str("/.skeleton/foo.toml");
    assert_eq!(actual, expected);
}
