use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

type SegmentMap = HashMap<char, HashSet<char>>;
type DigitToSegment = HashMap<u32, HashSet<char>>;

struct Mapping {
    map: SegmentMap,
    digit_to_segment: DigitToSegment,
}

impl Mapping {
    fn new(digit_to_segment: &DigitToSegment) -> Self {
        let segments = "abcdefg";
        Self {
            map: segments
                .chars()
                .map(|c| (c, segments.chars().collect()))
                .collect(),
            digit_to_segment: digit_to_segment.clone(),
        }
    }
    fn reduce(&mut self, digit: u32, set: HashSet<char>) {
        todo!()
    }
}

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

    let digit_to_segment = HashMap::from([
        (0, "abcefg"),
        (1, "cf"),
        (2, "acdeg"),
        (3, "acdfg"),
        (4, "bcdf"),
        (5, "abdfg"),
        (6, "abdefg"),
        (7, "acf"),
        (8, "abcdefg"),
        (9, "abcdfg"),
    ])
    .iter()
    .map(|(&k, v)| (k as u32, v.chars().collect()))
    .collect::<DigitToSegment>();

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
