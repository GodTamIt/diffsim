extern crate clap;
use clap::Shell;
use std::env;

include!("src/cli.rs");

fn main() {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return,
        Some(out_dir) => out_dir,
    };
    let bin_name = env!("CARGO_PKG_NAME");
    let mut app = build_cli();

    app.gen_completions(bin_name, Shell::Bash, &out_dir);
    app.gen_completions(bin_name, Shell::Fish, &out_dir);
}