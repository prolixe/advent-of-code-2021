use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: FromStr,
{
    fn from((x, y): (T, T)) -> Self {
        Point { x, y }
    }
}

/*Here lies my attempt at using try_from.
*Unwrap couldn't be called because of it not implementing
the correct error type or something similar*/
//impl<T> TryFrom<&str> for Point<T>
//where
//    T: FromStr,
//{
//    type Error = &'static str;
//    fn try_from(input: &str) -> Result<Self, Self::Error> {
//        let result = input.trim().split_once(',');
//        if let Some((x, y)) = result {
//            let x = x.parse::<T>().unwrap();
//            let y = y.parse::<T>();
//            Ok(Point::from((x, y)))
//        } else {
//            Err("failed to parse")
//        }
//    }
//}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Segment {
    start: Point<i32>,
    end: Point<i32>,
}

impl Segment {
    fn from_str(input: &str) -> Result<Self, String> {
        let (p1, p2) = input
            .split_once(" -> ")
            .expect("Failed to parse into points");
        let p1 = p1.trim().split_once(',').expect("can't split p1");
        let p2 = p2.trim().split_once(',').expect("can't split p2");

        let p1 = (p1.0.parse::<i32>().unwrap(), p1.1.parse::<i32>().unwrap());
        let p2 = (p2.0.parse::<i32>().unwrap(), p2.1.parse::<i32>().unwrap());
        Ok(Self {
            start: Point::from(p1),
            end: Point::from(p2),
        })
    }

    fn generate_points(&self) -> Vec<Point<i32>> {
        // Return either a vector or an iterator?
        // Should learn iterator
        if self.start.x == self.end.x {
            let start = cmp::min(self.start.y, self.end.y);
            let end = cmp::max(self.start.y, self.end.y);
            return (start..=end)
                .map(|y| Point { x: self.start.x, y })
                .collect::<Vec<_>>();
        } else if self.start.y == self.end.y {
            let start = cmp::min(self.start.x, self.end.x);
            let end = cmp::max(self.start.x, self.end.x);
            return (start..=end)
                .map(|x| Point { y: self.start.y, x })
                .collect::<Vec<_>>();
        }
        // Calculate diagonal lines
        // Assume perfect diagonal
        let (start, end) = if self.start.x < self.end.x {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        };

        let next = if start.y < end.y {
            // Increase the y if the start y is lower
            |y: i32, i: usize| y + i as i32
        } else {
            |y: i32, i: usize| y - i as i32
        };

        return (start.x..=end.x)
            .enumerate()
            .map(|(c, x)| Point {
                x,
                y: next(start.y, c),
            })
            .collect::<_>();
    }
}

struct Diagram {
    map: HashMap<Point<i32>, i32>,
}

impl Diagram {
    fn new() -> Self {
        Diagram {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, segment: &Segment) {
        // Add all the integer point between 2 segments
        let points = segment.generate_points();
        points.iter().for_each(|&p| {
            *self.map.entry(p.clone()).or_insert(0) += 1;
        })
    }
}

pub fn day_05() -> Result<(), String> {
    //let contents = read_file("./resources/day05_small.txt").expect("Could not open file");
    let contents = read_file("./resources/day05.txt").expect("Could not open file");
    let segments = parse_into_segments(&contents)?;

    let mut diagram = Diagram::new();

    segments.iter().for_each(|d| diagram.add(d));

    let overlap_count = diagram.map.iter().filter(|(_, c)| **c > 1).count();
    println!("Overlap count: {}", overlap_count);

    return Ok(());
}

fn read_file(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_into_segments(input: &str) -> Result<Vec<Segment>, String> {
    let segments = input
        .trim()
        .lines()
        .map(|l| Segment::from_str(l))
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();
    Ok(segments)
}
