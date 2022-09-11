use std::error::Error;
use std::{fmt, vec};

use crate::util;

#[derive(Debug)]
enum FoldDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
struct FoldDirectionParseError;
impl fmt::Display for FoldDirectionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid fold direction parameter")
    }
}

impl Error for FoldDirectionParseError {}

impl FoldDirection {
    fn parse(s: &str) -> Result<Self> {
        match s {
            "x" => Ok(FoldDirection::Vertical),
            "y" => Ok(FoldDirection::Horizontal),
            _ => Err(Box::new(FoldDirectionParseError {})),
        }
    }
}

#[derive(Debug)]
struct Fold {
    position: usize,
    direction: FoldDirection,
}

#[derive(Debug)]
struct Paper {
    dots: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl fmt::Display for Paper {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        self.dots.iter().for_each(|r| {
            r.iter().for_each(|bn| {
                let _ = write!(f, "{}", if *bn { "#" } else { "." });
            });
            let _ = writeln!(f);
        });
        write!(f, "")
    }
}

type Folds = Vec<Fold>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn day_13() -> Result<()> {
    //let contents = util::read_file("./resources/day13_small.txt").expect("Could not open file");
    let contents = util::read_file("./resources/day13.txt").expect("Could not open file");

    //println!("contents: \n{}", contents);
    let (paper, folds) = parse(&contents)?;
    //println!("paper:\n{}", paper);
    println!("folds:\n{:?}", folds);
    let part_1_paper = apply_fold(&paper, folds.first().unwrap()).unwrap();
    //print!("paper:\n{}", paper);
    println!("Count (part 1): {}", part_1_paper.count_dots());
    // That the 'fold' function is super appropriate for folding a transparent
    // paper repeatedly must have been intentionnal.
    let final_paper = folds.iter().fold(paper, |p, f| apply_fold(&p, f).unwrap());

    print!("Paper (part 2):\n{}", final_paper);
    Ok(())
}

fn parse(contents: &str) -> Result<(Paper, Folds)> {
    // Split the content into 2 parts
    let (paper_content, folds_content) = contents.trim().split_once("\n\n").unwrap();

    let paper = Paper::parse(paper_content)?;
    let folds = parse_instructions(folds_content)?;
    Ok((paper, folds))
}

impl Paper {
    fn parse(contents: &str) -> Result<Self> {
        let dots: Vec<(usize, usize)> = contents
            .trim()
            .split('\n')
            .map(|line| line.split_once(',').unwrap())
            .map(|(str_x, str_y)| (str_x.parse().unwrap(), str_y.parse().unwrap()))
            .collect();
        let width = *dots.iter().map(|(x, _y)| x).max().expect("No data in dots") + 1;
        let height = *dots.iter().map(|(_x, y)| y).max().expect("No data in dots") + 1;
        let mut dot_array = vec![vec![false; width]; height];
        for (x, y) in dots.into_iter() {
            dot_array[y][x] = true;
        }

        Ok(Paper {
            dots: dot_array,
            width,
            height,
        })
    }

    fn count_dots(&self) -> usize {
        self.dots
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }
}

fn parse_instructions(contents: &str) -> Result<Folds> {
    Ok(contents
        .trim()
        .split('\n')
        .map(|line| parse_instruction(line).unwrap())
        .collect::<Vec<Fold>>())
}

fn parse_instruction(line: &str) -> Result<Fold> {
    let mut words = line.split(' ');
    words.next(); // fold
    words.next(); // along
    let instruction = words.next().unwrap();
    let (direction, position) = instruction.split_once('=').expect("missing =");
    Ok(Fold {
        position: position.parse().unwrap(),
        direction: FoldDirection::parse(direction)?,
    })
}

fn apply_fold(paper: &Paper, fold: &Fold) -> Result<Paper> {
    // Find the dimensions of the new paper after being folded
    let new_height = match fold.direction {
        FoldDirection::Horizontal => fold.position,
        FoldDirection::Vertical => paper.height,
    };
    let new_width = match fold.direction {
        FoldDirection::Horizontal => paper.width,
        FoldDirection::Vertical => fold.position,
    };

    let mut new_dot_array = vec![vec![false; new_width]; new_height];
    /*
    println!(
        "new dimensions: heigth: {} width: {}",
        new_height, new_width
    );
    */

    paper.dots.iter().enumerate().for_each(|(row, r)| {
        r.iter().enumerate().for_each(|(col, dot)| {
            //println!("row, col: {},{}", row, col);
            let adjusted_col = if col > new_width - 1 {
                //(paper.width - 1) - col
                fold.position - (col - fold.position)
            } else {
                col
            };
            let adjusted_row = if row > new_height - 1 {
                //(paper.height - 1) - row
                //row - (row - (new_height - 1))
                fold.position - (row - fold.position)
            } else {
                row
            };
            //println!(
            //    "adjusted_row, adjusted_col: {},{}",
            //    adjusted_row, adjusted_col
            //);
            if adjusted_row < new_height && adjusted_col < new_width {
                new_dot_array[adjusted_row][adjusted_col] |= *dot;
            }
        });
    });

    Ok(Paper {
        dots: new_dot_array,
        height: new_height,
        width: new_width,
    })
}

#[test]
fn test_small_example_first_fold() {
    let input = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
    "
    .trim();

    let expected_folded_once = "
#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###
...........
...........
    "
    .trim();
    let (paper, folds) = parse(input).unwrap();
    let paper_folded_once = apply_fold(&paper, folds.first().unwrap()).unwrap();
    assert_eq!(paper_folded_once.count_dots(), 17);
    assert_eq!(
        format!("{}", paper_folded_once).trim(),
        expected_folded_once
    );
}
