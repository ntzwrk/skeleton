extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();
    app.gen_completions("skeleton", Shell::Bash, "completions/");
    app.gen_completions("skeleton", Shell::Fish, "completions/");
}
