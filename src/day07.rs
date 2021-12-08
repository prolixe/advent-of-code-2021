use std::fs::File;
use std::io::prelude::*;

type CrabVec = Vec<u32>;

pub fn day_07() -> Result<(), String> {
    //let contents = read_file("./resources/day07_small.txt").expect("Could not open file");
    let contents = read_file("./resources/day07.txt").expect("Could not open file");

    println!("contents: {}", contents);
    let crab_positions = parse_contents(&contents);
    println!("crabs: {:?}", crab_positions);

    println!("mean of crab pos: {:?}", mean(&crab_positions));
    println!("mean of crab pos: {:?}", median(&crab_positions));

    let min = crab_positions.iter().min().unwrap().clone();
    let max = crab_positions.iter().max().unwrap().clone();
    /*
    for pos in min..=max {
        println!(
            "fuel cost for pos {:?}= {}",
            pos,
            calculate_fuel_need(&crab_positions, pos)
        );
    }
    */

    let min_fuel_needed = (min..=max)
        .map(|pos| calculate_fuel_need(&crab_positions, pos))
        .min()
        .unwrap();
    println!("Min fuel required: {}", min_fuel_needed);
    let min_fuel_needed_v2 = (min..=max)
        .map(|pos| calculate_fuel_need_v2(&crab_positions, pos))
        .min()
        .unwrap();
    println!("Min fuel required v2: {}", min_fuel_needed_v2);

    return Ok(());
}

fn read_file(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_contents(input: &str) -> CrabVec {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn calculate_fuel_need(crabs: &CrabVec, destination: u32) -> u32 {
    let abs_diffs = crabs
        .iter()
        .map(|&p| (p as i64) - (destination as i64))
        .map(|diff| diff.abs() as u32)
        .sum::<u32>();
    return abs_diffs;
}

fn calculate_fuel_need_v2(crabs: &CrabVec, destination: u32) -> u32 {
    let abs_diffs = crabs
        .iter()
        .map(|&p| (p as i64) - (destination as i64))
        .map(|diff| diff.abs() as u32)
        .map(triangular_number)
        .sum::<u32>();
    return abs_diffs;
}

fn triangular_number(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn mean(numbers: &[u32]) -> f64 {
    let sum = numbers.iter().sum::<u32>() as f64;
    let count = numbers.len();

    let mean = sum / count as f64;
    mean
}

fn median(numbers: &[u32]) -> f64 {
    let len = numbers.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&numbers[(mid - 1)..(mid + 1)])
    } else {
        f64::from(numbers[mid])
    }
}
