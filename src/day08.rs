use std::fs::File;
use std::io::prelude::*;

pub fn day_08() -> Result<(), String> {
    let contents = read_file("./resources/day08_small.txt").expect("Could not open file");
    //let contents = read_file("./resources/day08.txt").expect("Could not open file");

    println!("contents: {}", contents);

    let patterns_and_outputs = parse_patterns_and_output(&contents);
    let digits_1478: usize = patterns_and_outputs
        .iter()
        .map(|(_, output)| output)
        .map(|o| find_easy_digits(o).len())
        .sum();
    println!("digits_1478 count: {:?}", digits_1478);

    return Ok(());
}

fn read_file(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_patterns_and_output(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .trim()
        .split('\n')
        .map(|s| {
            let (pattern, output) = s.split_once('|').unwrap();
            (parse_pattern(pattern), parse_output(output))
        })
        .collect()
}

fn parse_pattern(input: &str) -> Vec<&str> {
    input.trim().split(' ').collect()
}

fn parse_output(input: &str) -> Vec<&str> {
    input.trim().split(' ').collect()
}

fn find_easy_digits(outputs: &Vec<&str>) -> Vec<u32> {
    outputs
        .iter()
        .filter_map(|s| match s.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        })
        .collect()
}
