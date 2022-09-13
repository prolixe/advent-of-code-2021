use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn read_file(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;
