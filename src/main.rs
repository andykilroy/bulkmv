#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {
        foreign_links {
            // adapt io errors to this module's errors
            Io(::std::io::Error) #[cfg(unix)];
        }

        errors {
            NonCorresponding(t: String) {
                display("src contents don't match dest contents -- {}", t)
            }
            RepeatedSrcPaths(t: String) {
                display("some source paths are identical, or only differ in case, e.g. {}", t)
            }
            RepeatedDestPaths(t: String) {
                display("some dest paths are identical, or only differ in case, e.g. {}", t)
            }
        }
    }

}

use errors::*;

extern crate clap;
use clap::{Arg, App, SubCommand};
use std::process;
use std::path::Path;
use std::fs::*;
use std::io::BufReader;
use std::io::BufRead;
use std::string::String;
use std::collections::BTreeSet;




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

fn move_all<P: AsRef<Path>, Q: AsRef<Path>>(srcfiles: &[P], destfiles: &[Q]) -> Result<()> {
    let len = srcfiles.len();
    if len != destfiles.len() {
        return Err(Error::from_kind(ErrorKind::NonCorresponding(format!("src lines: {}, dest lines: {}", len, destfiles.len()))));
    }
    assert_distinct_paths(true, srcfiles)?;
    assert_distinct_paths(false, destfiles)?;

    for i in 0..len {
        ensure_dest_dir_exists(&destfiles[i]);
        std::fs::rename(&srcfiles[i], &destfiles[i])?;
    }
    Ok(())
}

fn ensure_dest_dir_exists<P: AsRef<Path>>(file: P) -> Result<()> {
    if let Some(parent) = file.as_ref().parent() {
        create_dir_all(parent)?;
    }
    Ok(())
}

fn assert_distinct_paths<P: AsRef<Path>>(are_source_paths: bool, paths: &[P]) -> Result<()> {
    let mut s = BTreeSet::<String>::new();
    for p in paths {
        if let Some(orig) = p.as_ref().to_str() {
            s.insert(orig.to_ascii_lowercase());
        }
    }
    if s.len() != paths.len() {
        if are_source_paths {
            return Err(Error::from_kind(ErrorKind::RepeatedSrcPaths(format!(""))));
        } else {
            return Err(Error::from_kind(ErrorKind::RepeatedDestPaths(format!(""))));
        }
    }
    Ok(())
}

fn contents_of<P: AsRef<Path>>(filepath: P) -> Result<Vec<String>> {
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);
    let mut pathlist: Vec<String> = vec![];
    for pathstr in reader.lines() {
        pathlist.push(pathstr?);
    }
    Ok(pathlist)
}

