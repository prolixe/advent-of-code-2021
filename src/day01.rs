use std::fs::File;
use std::io::prelude::*;

pub fn day_01() -> std::io::Result<()> {
    // let mut file = File::open("./resources/day01_small.txt")?;
    let mut file = File::open("./resources/day01.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("contents:\n{}", contents);

    let report: Vec<i32> = contents
        .trim()
        .split('\n')
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect();

    let increase_count = report
        .windows(2)
        .filter(|entries| entries[0] < entries[1])
        .count();

    println!(
        "There are {} measurements that are larger than the previous one",
        increase_count,
    );

    return Ok(());
}
