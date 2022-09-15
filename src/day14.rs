use crate::util::Result;

use std::collections::HashMap;
use std::convert::TryInto;
use std::usize::MAX;

type Pair = (u8, u8);

#[derive(Debug, PartialEq)]
struct Polymer {
    map: HashMap<Pair, usize>,
    last_elem: u8,
}

#[derive(Debug)]
struct Rules {
    map: HashMap<Pair, u8>,
}

pub fn day_14() -> Result<()> {
    let contents = include_str!("../resources/day14.txt");
    let (mut polymer, rules) = parse(contents)?;
    for _ in 0..10 {
        polymer = step(&polymer, &rules).unwrap();
    }
    println!("Part 1: {:?}", diff_most_and_least_common_elem(&polymer));
    for _ in 10..40 {
        polymer = step(&polymer, &rules).unwrap();
    }
    println!("Part 2: {:?}", diff_most_and_least_common_elem(&polymer));
    Ok(())
}

fn parse(contents: &str) -> Result<(Polymer, Rules)> {
    let (template, rules) = contents.trim().split_once("\n\n").unwrap();
    let rules = Rules::parse(rules);
    let polymer = Polymer::parse(template).unwrap();
    Ok((polymer, rules))
}

fn count_elements(polymer: &Polymer) -> HashMap<u8, usize> {
    // Take the first element of each "pairs", since they are forming a long
    // link. Add the last element at the end, since it's not the start of a pair
    let last_elem = polymer.last_elem;
    let mut elem_count: HashMap<u8, usize> = HashMap::new();

    for (pair, count) in polymer.map.iter() {
        elem_count
            .entry(pair.0)
            .and_modify(|c| *c += count)
            .or_insert(*count);
    }

    elem_count.entry(last_elem).and_modify(|count| *count += 1);
    elem_count
}

fn diff_most_and_least_common_elem(polymer: &Polymer) -> usize {
    let counted_elem = count_elements(polymer);
    let mut min = MAX;
    let mut max = 0usize;
    for (_, count) in counted_elem.into_iter() {
        if count > max {
            max = count;
        }
        if count < min {
            min = count;
        }
    }
    max - min
}

impl Rules {
    fn parse(input: &str) -> Self {
        let hashmap = input
            .trim()
            .split('\n')
            .into_iter()
            .map(|line| {
                let (adjacent_elem, out_elem) = line.split_once(" -> ").unwrap();
                assert!(adjacent_elem.len() >= 2);
                (
                    (adjacent_elem.as_bytes()[0], adjacent_elem.as_bytes()[1]),
                    out_elem.as_bytes()[0],
                )
            })
            .collect();
        Self { map: hashmap }
    }

    fn apply(&self, pair: &Pair) -> Option<(Pair, Pair)> {
        let new_element = self.map.get(pair);
        new_element.copied().map(|e| ((pair.0, e), (e, pair.1)))
    }
}

impl Polymer {
    fn new(last_elem: u8) -> Self {
        Polymer {
            map: HashMap::new(),
            last_elem,
        }
    }

    fn parse(content: &str) -> Result<Self> {
        let map_tuple = content
            .trim()
            .as_bytes()
            .windows(2)
            .map(|pair| (pair[0], pair[1]));

        let last_elem = *content.as_bytes().last().unwrap();
        let mut polymer = Self::new(last_elem);
        for pair in map_tuple {
            polymer.add(&pair, 1);
        }

        Ok(polymer)
    }

    fn add(&mut self, pair: &Pair, times: usize) {
        self.map
            .entry(*pair)
            .and_modify(|counter| *counter += times)
            .or_insert(times);
    }
}

fn step(polymer: &Polymer, rules: &Rules) -> Result<Polymer> {
    // Every pair produce 2 new pairs according to the rules and produce a new
    // polymer.
    let mut new_polymer = Polymer::new(polymer.last_elem);

    for (pair, count) in polymer.map.iter() {
        let maybe_pairs = rules.apply(pair);
        if let Some(pairs) = maybe_pairs {
            new_polymer.add(&pairs.0, *count);
            new_polymer.add(&pairs.1, *count);
        }
    }

    Ok(new_polymer)
}

#[test]
fn apply_first_step() {
    let contents = include_str!("../resources/day14_small.txt");

    let (mut polymer, rules) = parse(contents).unwrap();
    polymer = step(&polymer, &rules).unwrap();

    let expect_polymer = Polymer::parse("NCNBCHB").unwrap();
    assert_eq!(polymer, expect_polymer);
}

#[test]
fn apply_4_steps() {
    let contents = include_str!("../resources/day14_small.txt");
    let (mut polymer, rules) = parse(contents).unwrap();
    for _ in 0..4 {
        polymer = step(&polymer, &rules).unwrap();
    }

    let expect_polymer =
        Polymer::parse("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB").unwrap();
    assert_eq!(polymer, expect_polymer);
}

#[test]
fn count_polymer() {
    let polymer = Polymer::parse("NNCB").unwrap();

    let count = count_elements(&polymer);

    let n = 'N'.try_into().unwrap();
    assert_eq!(*count.get(&n).unwrap(), 2usize)
}

#[test]
fn apply_10_steps() {
    let contents = include_str!("../resources/day14_small.txt");
    let (mut polymer, rules) = parse(contents).unwrap();
    for _ in 0..10 {
        polymer = step(&polymer, &rules).unwrap();
    }
    let diff = diff_most_and_least_common_elem(&polymer);

    assert_eq!(diff, 1588);
}
