use std::fs::File;
use std::io::prelude::*;

pub fn day_01() -> std::io::Result<()> {
    let mut file = File::open("./resources/day01_small.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("contents:\n{}", contents);

    return Ok(());
}
