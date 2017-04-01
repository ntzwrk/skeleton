# Skeleton

[![Build Status](https://travis-ci.org/ntzwrk/skeleton.svg?branch=master)](https://travis-ci.org/ntzwrk/skeleton)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/ntzwrk/skeleton/blob/master/LICENSE)

Skeleton is a management tool for project prototypes. Prototypes are defined in language specific toml files. Skeleton can create directories, touch files, execute predefined commands and download a .gitignore list from [gitignore.io](https://gitignore.io).

Skeleton is written in pure Rust because I wanted to learn this language using a small hobby project.

## Configuration

Language specific configuration must be placed in `${HOME}/.skeleton` and are referenced by their name without the `.toml` extension.

### Configuration format

```toml
mkdir = ['src', 'test']
touch = ['README.md']
exec = ['cargo init']
gitignore = ['rust', 'vim']
```

## Usage
```
Skeleton 0.1
Valentin B. <mail@mail.mail>
Skeleton project manager

USAGE:
    skeleton --lang <LANG> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lang <LANG>    Set language configuration

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    init    initialize existing project
    new     create new project
```

So to initialize a new Rust project named `test_project` one would execute `skeleton -l rust new test_project`. Therefore a configuration file `$HOME/.skeleton/rust.toml` must exist.
