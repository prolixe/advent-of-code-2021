use std::fs::File;
use std::io::prelude::*;

pub fn day_05() -> Result<(), String> {
    let contents = read_file("./resources/day05_small.txt").expect("Could not open file");
    //let contents = read_file("./resources/day05.txt").expect("Could not open file");

    return Ok(());
}

fn read_file(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("contents:\n{}", contents);
    Ok(contents)
}
