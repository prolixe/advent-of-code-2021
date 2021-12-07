use std::fmt;
use std::fs::File;
use std::io::prelude::*;

struct Fish {
    timer: u32,
}

// Number of fish in a certain state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FishState {
    fish_count: u64, // There could be a _lot_ of fishes
}

#[derive(Debug)]
struct FishPopState {
    day: u32,
    fishes: Vec<FishState>,
}

impl FishPopState {
    fn new() -> Self {
        FishPopState {
            day: 0,
            fishes: Self::new_generation(),
        }
    }

    fn new_generation() -> Vec<FishState> {
        vec![FishState { fish_count: 0 }; 9]
    }
    // Add a fish to the population
    fn add_fish(&mut self, fish: Fish) {
        assert!(fish.timer < 9);

        self.fishes[fish.timer as usize].fish_count += 1;
    }

    // Advance the simulation for a day
    fn step(&mut self) {
        self.day += 1;
        // Decrement all the TimerState and process timerState at 0

        let mut next_generation = Self::new_generation();

        // Move a whole generation down 1 tick
        self.fishes.iter().enumerate().for_each(|(c, f)| {
            if c != 0 {
                next_generation[c - 1] = f.clone();
            }
        });

        // Add new fish from timer0 to timer8
        let born_fish_count = self.fishes[0].fish_count;
        next_generation[8] = FishState {
            fish_count: born_fish_count,
        };

        // Merge fish from timer0 to timer6
        next_generation[6] = FishState {
            fish_count: next_generation[6].fish_count + born_fish_count,
        };

        // Replace the old generation with the new
        self.fishes = next_generation;
    }

    fn count(&self) -> u64 {
        // Loop over all the fishes and get the number of fish in that each state
        self.fishes.iter().map(|f| f.fish_count).sum()
    }
}

impl fmt::Display for FishPopState {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "FishPopState: day {}, pop: {}", self.day, self.count())
    }
}
pub fn day_06() -> Result<(), String> {
    //let contents = read_file("./resources/day06_small.txt").expect("Could not open file");
    let contents = read_file("./resources/day06.txt").expect("Could not open file");

    println!("contents: {}", contents);
    let mut fish_pop = FishPopState::new();
    contents
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .map(|d| Fish { timer: d })
        .for_each(|f| fish_pop.add_fish(f));

    println!("Fish pop day: {}", fish_pop);
    let max_days = 80;
    for i in 1..=max_days {
        fish_pop.step();
        println!("Fish pop day: {}", fish_pop);
    }

    return Ok(());
}

fn read_file(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
