use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<u8>>,
}

impl HeightMap {
    fn from_str(input: &str) -> Self {
        HeightMap {
            map: input
                .trim()
                .split('\n')
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect::<Vec<Vec<u8>>>(),
        }
    }

    fn find_low_points(&self) -> Vec<Point> {
        // Low point if adjacent points are all higher
        let dim = self.dimensions();
        (0..dim.y)
            .flat_map(|y| (0..dim.x).map(move |x| Point { x, y }))
            .filter(|p| self.is_low_points(p))
            .collect()
    }

    fn find_basin(&self, low_point: &Point) -> HashSet<Point> {
        let mut set: HashSet<Point> = HashSet::new();
        set.insert(low_point.clone());

        self.find_basin_pos_recursive(&mut set)
    }

    fn find_basin_pos_recursive(&self, set: &mut HashSet<Point>) -> HashSet<Point> {
        let mut basin = set
            .iter()
            .cloned()
            .flat_map(|p| self.adjacent_points(&p))
            .filter(|p| self.val(p) != 9) // 9 are not part of a basin
            .collect::<HashSet<Point>>();

        basin = basin.union(set).cloned().collect();
        let new_points: HashSet<_> = basin.difference(set).cloned().collect();
        //println!("set: {:?}", set);
        //println!("basin: {:?}", basin);
        //println!("new_points: {:?}", new_points);
        if new_points.is_empty() {
            // No new points mean we reached the max size of the basin
            basin
        } else {
            self.find_basin_pos_recursive(&mut basin)
        }
    }

    fn low_point_risk_level(&self, p: &Point) -> u8 {
        self.val(p) + 1
    }

    fn is_low_points(&self, p: &Point) -> bool {
        let adjacents = self.adjacent_points(&p);

        let point_val = self.val(&p);
        adjacents.iter().all(|a| point_val < self.val(a))
    }

    fn val(&self, p: &Point) -> u8 {
        self.map[p.y][p.x]
    }

    fn adjacent_points(&self, p: &Point) -> Vec<Point> {
        let mut adjacents: Vec<Point> = Vec::new();
        let max = self.dimensions();
        // Point over
        if p.y >= 1 {
            adjacents.push(Point { x: p.x, y: p.y - 1 });
        }
        // Point under
        if p.y < max.y - 1 {
            adjacents.push(Point { x: p.x, y: p.y + 1 });
        }
        // Point left
        if p.x >= 1 {
            adjacents.push(Point { x: p.x - 1, y: p.y });
        }

        // Point right
        if p.x < max.x - 1 {
            adjacents.push(Point { x: p.x + 1, y: p.y });
        }
        return adjacents;
    }

    fn dimensions(&self) -> Point {
        assert!(self.map.len() > 0);
        Point {
            x: self.map[0].len(),
            y: self.map.len(),
        }
    }
}

pub fn day_09() -> Result<(), String> {
    //let contents = read_file("./resources/day09_small.txt").expect("Could not open file");
    let contents = read_file("./resources/day09.txt").expect("Could not open file");

    println!("contents: \n{}", contents);

    let height_map = HeightMap::from_str(&contents);
    let risk_level_sum: u32 = height_map
        .find_low_points()
        .iter()
        .map(|p| height_map.low_point_risk_level(p) as u32)
        .sum();

    println!("risk_level_sum: {:?}", height_map.find_low_points());
    println!(
        "val: {:?}",
        height_map
            .find_low_points()
            .iter()
            .map(|p| height_map.val(p))
            .collect::<Vec<u8>>()
    );
    println!("risk_level_sum: {}", risk_level_sum);

    let mut basin_sizes = height_map
        .find_low_points()
        .iter()
        .map(|p| height_map.find_basin(p).len())
        .collect::<Vec<usize>>();

    println!("basin_sizes: {:?}", basin_sizes);
    basin_sizes.sort();

    println!(
        "basin 3 biggest sum: {:?}",
        &basin_sizes[(basin_sizes.len() - 3)..]
            .iter()
            .product::<usize>()
    );
    return Ok(());
}

fn read_file(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
