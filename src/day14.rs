use crate::util::Result;

use std::slice::from_ref;
use std::str::from_utf8;
use std::usize::MAX;
use std::{collections::HashMap, convert::TryInto};

type Pair = (u8, u8);
type Polymer = Vec<u8>;

#[derive(Debug)]
struct Rules {
    map: HashMap<Pair, u8>,
}

pub fn day_14() -> Result<()> {
    let contents = include_str!("../resources/day14.txt");
    println!("{}", contents);
    let (mut polymer, rules) = parse(contents)?;
    println!("{:?}", polymer);
    println!("{:?}", rules);
    for _ in 0..10 {
        polymer = step(&polymer, &rules).unwrap();
    }
    println!("{:?}", from_utf8(&polymer));
    println!("{:?}", diff_most_and_least_common_elem(&polymer));

    Ok(())
}

fn parse(contents: &str) -> Result<(Polymer, Rules)> {
    let (template, rules) = contents.trim().split_once("\n\n").unwrap();
    let rules = Rules::parse(rules);
    Ok((template.as_bytes().to_vec(), rules))
}

fn count_elements(polymer: &Polymer) -> HashMap<&u8, usize> {
    let mut hashmap = HashMap::new();
    for elem in polymer {
        if hashmap.contains_key(elem) {
            *hashmap.get_mut(elem).unwrap() += 1;
        } else {
            hashmap.insert(elem, 1);
        }
    }
    hashmap
}

fn diff_most_and_least_common_elem(polymer: &Polymer) -> usize {
    let counted_elem = count_elements(polymer);
    let mut min = MAX;
    let mut max = 0usize;
    for (elem, count) in counted_elem.into_iter() {
        if count > max {
            max = count;
        }
        if count < min {
            min = count;
        }
    }
    (max - min)
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

    fn apply(&self, pair: &Pair) -> Option<Vec<u8>> {
        self.map.get(pair).map(|elem| vec![pair.0, *elem])
    }
}

fn step(polymer: &Polymer, rules: &Rules) -> Result<Polymer> {
    let mut new_polymer: Polymer = polymer
        .windows(2)
        .flat_map(|pair| {
            rules
                .apply(&(pair[0], pair[1]))
                .unwrap_or_else(|| pair.to_vec())
        })
        .collect();
    let last_elem = polymer.get(polymer.len() - 1).unwrap();
    new_polymer.push(*last_elem);
    Ok(new_polymer)
}

#[test]
fn apply_first_step() {
    let contents = include_str!("../resources/day14_small.txt");
    let (polymer, rules) = parse(contents).unwrap();

    let step_1 = step(&polymer, &rules).unwrap();
    assert_eq!(from_utf8(&step_1).unwrap(), "NCNBCHB");
}

#[test]
fn apply_4_steps() {
    let contents = include_str!("../resources/day14_small.txt");
    let (mut polymer, rules) = parse(contents).unwrap();

    for _ in 0..4 {
        polymer = step(&polymer, &rules).unwrap();
    }
    assert_eq!(
        from_utf8(&polymer).unwrap(),
        "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
    );
}
