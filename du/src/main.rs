/*

du unix codeutils implementation in rust.

reference https://github.com/coreutils/coreutils/blob/master/src/du.c

1. Dirty Implementation.
args: no args at the moment.
- at the current directory
- iterate over everything in that directory 
- if anything is a folder 
    - go inside that and run loop again
- if anything is a file
    - total += file size
- print total

**/
use std::{env, path::PathBuf};
use std::io::Result;
use std::fs;

fn walk(dir_path: PathBuf) -> Result<i64> {
    let mut size = 0;

    for entry in dir_path.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            size = size + walk(path)?;
        } else if path.is_file() {
            size += fs::metadata(&path)?.len() as i64;
        }
    }

    Ok(size)
}

fn main() -> Result<()> {
    let current_path = env::current_dir().unwrap();

    let total_size = walk(current_path)?;

    println!("total size {}", total_size);

    Ok(())
}