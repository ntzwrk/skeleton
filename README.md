# Skeleton

[![Build Status](https://travis-ci.org/ntzwrk/skeleton.svg?branch=master)](https://travis-ci.org/ntzwrk/skeleton)
[![codecov.io](http://codecov.io/github/ntzwrk/editorconfig.me/skeleton.svg?branch=master)](http://codecov.io/github/ntzwrk/skeleton?branch=master)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/ntzwrk/skeleton/blob/master/LICENSE)
[![crates.io](https://meritbadge.herokuapp.com/skeleton)](https://crates.io/crates/skeleton)

Skeleton is a management tool for project prototypes. Prototypes are defined in language specific toml files. Skeleton can create directories, touch files, execute predefined commands and download a .gitignore list from [gitignore.io](https://gitignore.io).

Skeleton is written in pure Rust because I wanted to learn this language using a small hobby project.

## Installation

To install skeleton, you need the Rust package manager [cargo](https://github.com/rust-lang/cargo).

```
cargo install skeleton
```

To install the most current version from master (might be buggy):

```
git clone https://github.com/ntzwrk/skeleton.git
cd skeleton
cargo install
```

## Configuration

Language specific configuration must be placed in `${HOME}/.skeleton` and are referenced by their name without the `.toml` extension.

### Configuration format

```toml
order = ['mkdir', 'touch', 'exec', 'gitignore']
mkdir = ['src', 'test']
touch = ['README.md']
exec = ['cargo init']
gitignore = ['rust', 'vim']
include = ['global']
```

Every configuration setting is optional. The `order` setting is used to customize the execution order. The default
order is `mkdir`, `gitignore`, `touch`, `exec`.

Includes are executed first and in the provided order, followed by the selected configuration.

## Usage
```
Skeleton 0.2.1
Valentin B. <vbrandl@riseup.net>
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

## Shell completions

Shell completions for [Bash](https://www.gnu.org/software/bash/), [Fish](https://github.com/fish-shell/fish-shell), [Zsh](https://www.zsh.org/) and PowerShell can be found in the `completions` folder.
