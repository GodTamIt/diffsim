use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
            .about("An advanced file similarity analysis tool")
            .version(env!("CARGO_PKG_VERSION"))
            .subcommand(SubCommand::with_name("bash-completions")
                .about("Prints command completions for the Bash shell"))
            .subcommand(SubCommand::with_name("pair")
                .about("Computes a similarity metric between two files")
                .arg(Arg::with_name("FILE1")
                    .help("The first file to compare")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("FILE2")
                    .help("The second file to compare")
                    .required(true)
                    .index(2)))
}