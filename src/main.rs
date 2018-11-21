#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

use errors::*;

extern crate clap;
use clap::{Arg, App, SubCommand};
use std::process;
use std::path::Path;



fn main() {
    if let Err(e) = start_app() {
        eprintln!("Error: {}", e);
        for i in e.iter().skip(1) {
            eprintln!("Caused by: {}", i);
        }
        process::exit(1);
    }
}

fn start_app() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Move files as dictated by the contents of this command's two input files")
        .arg(Arg::with_name("SRCFILE")
                      .help("Each line in this file is a file to be moved.")
                      .required(true)
                      .index(1))
        .arg(Arg::with_name("DESTFILE")
                      .help("Each line in this file dictates where the corresponding file in SRCFILE gets moved to.")
                      .required(true)
                      .index(2))
        .get_matches();

    let src = matches.value_of("SRCFILE").expect("expected a sources file");
    let dest = matches.value_of("DESTFILE").expect("expected a destinations file");

    let srcfiles = contents_of(src)?;
    let destfiles = contents_of(dest)?;

    move_all(&srcfiles, &destfiles)?;
    Ok(())
}

fn contents_of<P: AsRef<Path>>(file: P) -> Result<Vec<P>> {
    Ok(vec![])
}

fn move_all<P: AsRef<Path>, Q: AsRef<Path>>(srcfiles: &[P], destfiles: &[Q]) -> Result<()> {
    Ok(())
}
