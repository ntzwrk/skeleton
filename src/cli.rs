use clap::{App, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("Skeleton")
        .version("0.2.1")
        .author("Valentin B. <vbrandl@riseup.net>")
        .about("Skeleton project manager")
        .args_from_usage("-l, --lang=<LANG>  'Set language configuration'")
        .subcommand(SubCommand::with_name("new")
                        .about("create new project")
                        .arg_from_usage("<NAME>   'The project name'"))
        .subcommand(SubCommand::with_name("init").about("initialize existing project"))
}
