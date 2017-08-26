extern crate clap;
extern crate difflib;

mod cli;

use std::io;
use std::io::Read;
use std::fs::File;

use difflib::sequencematcher::SequenceMatcher;

fn read_file(file: &str) -> io::Result<String> {
    let mut contents = String::new();
    let mut f = try!(File::open(file));
    try!(f.read_to_string(&mut contents));

    Ok(contents)
}

fn get_ratio(s1: &String, s2: &String) -> f32 {
    let mut one_two_matcher = SequenceMatcher::new(s1, s2);
    let mut two_one_matcher = SequenceMatcher::new(s2, s1);

    return (one_two_matcher.ratio() + two_one_matcher.ratio()) / 2.0;
}

fn ratio(file1: &str, file2: &str) -> Result<(), ()> {
    let (s1, s2): (String, String);

    match read_file(file1) {
        Ok(s) => s1 = s,
        _ => { 
            println!("Error reading file: {}", file1);
            return Err(());
        },
    }

    match read_file(file2) {
        Ok(s)   => s2 = s,
        _       => {
            println!("Error reading file: {}", file2);
            return Err(());
        },
    }

    println!("{}", get_ratio(&s1, &s2));

    Ok(())
}

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(ref matches) = matches.subcommand_matches("pair") {
        // Safe to use unwrap() because of the required() option
        match ratio(matches.value_of("FILE1").unwrap(), matches.value_of("FILE2").unwrap()) {
            Ok(()) => std::process::exit(0),
            Err(()) => std::process::exit(1),
        }
    }
}
