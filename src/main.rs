extern crate clap;
extern crate difflib;
extern crate rayon;
extern crate strsim;

mod cli;
mod commands;
pub mod utils;

use std::io;

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(ref matches) = matches.subcommand_matches("score") {
        exit_code(commands::score(matches.value_of("file1").unwrap(), matches.value_of("file2").unwrap()));
    } else if let Some(ref matches) = matches.subcommand_matches("multiscore") {
        let file_list1 = matches.value_of("file_list1").unwrap();
        let mem = !matches.is_present("no-mem");
        match matches.value_of("file_list2") {
            Some(v) => exit_code(commands::multiscore_two_lists(file_list1, v, mem)),
            None    => exit_code(commands::multiscore_one_list(file_list1, mem)),
        }
    }
}

fn exit_code(result: io::Result<()>) {
    match result {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        },
    }
}