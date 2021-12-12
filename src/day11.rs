use crate::util;
use std::fmt;

#[derive(Default, Clone, Copy)]
struct Octopus {
    level: u32,
    flashing: bool,
}

impl Octopus {
    fn increase_level(&mut self) {
        self.level += 1;
    }

    fn over_threshold(&self) -> bool {
        self.level > 9
    }

    fn reset(&mut self) {
        if self.flashing {
            self.flashing = false;
            self.level = 0;
        }
    }

    fn flash(&mut self, map: &mut Map<Octopus>, pos: Vec2) {
        self.flashing = true;
        map.set(pos, self.clone());
        map.neighbor_positions(pos).for_each(|n_pos| {
            if let Some(mut n_o) = map.get(n_pos) {
                n_o.increase_level();
                map.set(n_pos, n_o);
                if !n_o.flashing && n_o.over_threshold() {
                    n_o.flash(map, n_pos);
                }
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64,
}

struct Map<T> {
    size: Vec2,
    octopi: Vec<T>,
}

impl<T> Map<T>
where
    T: Default + Copy,
{
    fn new(size: Vec2) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            octopi: (0..num_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }

    fn index(&self, pos: Vec2) -> Option<usize> {
        if (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y) {
            Some((pos.x + pos.y * self.size.x) as _)
        } else {
            None
        }
    }

    fn set(&mut self, pos: Vec2, octopus: T) {
        if let Some(index) = self.index(pos) {
            self.octopi[index] = octopus;
        }
    }

    fn get(&self, pos: Vec2) -> Option<T> {
        self.index(pos).map(|index| self.octopi[index])
    }

    fn neighbor_positions(&self, pos: Vec2) -> impl Iterator<Item = Vec2> {
        (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
            .map(move |(dx, dy)| Vec2 {
                x: pos.x + dx,
                y: pos.y + dy,
            })
    }
}

impl Map<Octopus> {
    fn step(&mut self) -> usize {
        // Increase level of all octopi by 1
        let (row, col) = (self.size.y, self.size.x);
        for c in 0..col {
            for r in 0..row {
                let pos = Vec2 { x: r, y: c };
                let mut o = self.get(pos).unwrap();
                o.increase_level();
                self.set(pos, o);
            }
        }
        for c in 0..col {
            for r in 0..row {
                let pos = Vec2 { x: r, y: c };
                let mut o = self.get(pos).unwrap();
                if o.over_threshold() && !o.flashing {
                    o.flash(self, pos);
                }
            }
        }

        let flash_count = self.octopi.iter().filter(|o| o.flashing).count();

        // Once no new neighbor flashes, loop over all octopis and set flashing ones to 0
        for c in 0..col {
            for r in 0..row {
                let pos = Vec2 { x: r, y: c };
                let mut o = self.get(pos).unwrap();
                o.reset();
                self.set(pos, o);
            }
        }

        flash_count
    }
}

impl fmt::Display for Map<Octopus> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let (row, col) = (self.size.y, self.size.x);
        for c in 0..col {
            for r in 0..row {
                let o = self.get(Vec2 { x: r, y: c }).unwrap();
                write!(f, "{}", o.level)?;
            }
            write!(f, "\n")?;
        }

        write!(f, "")
    }
}

fn parse(input: &str) -> Map<Octopus> {
    let octopi: Vec<_> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Octopus {
                        level: c.to_digit(10).unwrap(),
                        flashing: false,
                    },
                    Vec2 {
                        x: x as i64,
                        y: y as i64,
                    },
                )
            })
        })
        .flatten()
        .collect();

    let col = input
        .trim()
        .lines()
        .into_iter()
        .collect::<Vec<&str>>()
        .len();
    let row = input.trim().split_once('\n').unwrap().0.len();
    let size = Vec2 {
        x: row as i64,
        y: col as i64,
    };

    let mut map: Map<Octopus> = Map::new(size);

    octopi.iter().for_each(|(o, pos)| {
        map.set(*pos, *o);
    });
    map
}

pub fn day_11() -> Result<(), String> {
    let contents = util::read_file("./resources/day11_small.txt").expect("Could not open file");
    //let contents = util::read_file("./resources/day11.txt").expect("Could not open file");

    println!("contents: \n{}", contents);

    let mut map = parse(&contents);

    println!("octopi: \n{}", map);

    let mut total_flashes = 0;
    for i in 0..100 {
        total_flashes += map.step();
    }

    println!("octopi after 100 steps: \n{}", map);
    println!("Total flash: {}", total_flashes);

    return Ok(());
}

#[test]
fn test_neighbor_positions() {
    use std::collections::HashSet;

    let map = Map::<()>::new(Vec2 { x: 3, y: 3 });
    let positions: HashSet<_> = map
        .neighbor_positions(Vec2 { x: 1, y: 1 })
        .map(|v| (v.x, v.y))
        .collect();
    for p in &[
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (2, 0),
        (1, 2),
        (2, 2),
        (2, 1),
    ] {
        assert!(positions.contains(p));
    }
}

#[test]
fn test_step() {
    let input = "
11111
19991
19191
19991
11111
"
    .trim();

    let step_1 = "
34543
40004
50005
40004
34543
"
    .trim();
    let mut map = parse(&input);
    let flash_count = map.step();
    assert_eq!(flash_count, 9);
    assert_eq!(format!("{}", map).trim(), step_1);
}

#[test]
fn test_step_large() {
    let input = "
    5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"
    .trim();

    let step_1 = "
    6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637
"
    .trim();
    let mut map = parse(&input);
    let flash_count = map.step();
    assert_eq!(flash_count, 0);
    assert_eq!(format!("{}", map).trim(), step_1);
}
