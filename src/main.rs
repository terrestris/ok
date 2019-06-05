use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::env::{current_dir, args};
use std::io::Error;

fn append_okfile(cur: &Path, okfilename: &str) -> Option<String> {
    let mut contents = String::new();
    let mut path = PathBuf::from(cur);
    path.push(okfilename);
    let f: &mut Result<File, Error> = &mut File::open(path.as_path());
    match f {
        Ok(file) => match file.read_to_string(&mut contents) {
            Ok(_s) => {
                Some(contents)
            },
            Err(_e) => None
            },
        Err(_e) => None
    }
}

fn main() -> std::io::Result<()> {
    let okfilename = if args().len() == 1 { ".ok" } else { ".ok.fish" };
    let mut contents = String::new();
    let path = current_dir()?;
    let mut num = 0;
    for ancestor in path.ancestors() {
        match append_okfile(ancestor, okfilename) {
            Some(str) => {
                contents.push_str(&str);
            },
            None => ()
        };
    };
    for line in contents.lines() {
        print!("{} - {}\n", num, line);
        num += 1;
    }
    Ok(())
}