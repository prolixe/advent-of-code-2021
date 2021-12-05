use std::fmt;
use std::fs::File;
use std::io::prelude::*;

type DrawnNumbers = Vec<u32>;

#[derive(Debug)]
struct BingoNumber {
    number: u32,
    marked: bool,
}

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<BingoNumber>>,
}

impl fmt::Display for BingoBoard {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Board:\n");
        self.board.iter().for_each(|r| {
            r.iter().for_each(|bn| {
                write!(f, "{}\t", if bn.marked { "x" } else { " " });
            });
            write!(f, "\n");
        });
        write!(f, "")
    }
}

impl BingoBoard {
    fn from_string(input: &str) -> Result<Self, String> {
        Ok(BingoBoard {
            board: input
                .split('\n')
                .map(|lines| {
                    lines
                        .split(' ')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<u32>().unwrap())
                        .map(|n| BingoNumber {
                            number: n,
                            marked: false,
                        })
                        .collect::<Vec<BingoNumber>>()
                })
                .collect::<Vec<Vec<BingoNumber>>>(),
        })
    }

    fn is_bingo(&self) -> bool {
        // Check horizontal

        for i in 0..self.board.len() {
            let row_complete = self.board[i].iter().all(|bn| bn.marked);
            if row_complete {
                return row_complete;
            }
        }
        for i in 0..self.board[0].len() {
            let col_complete = self.board.iter().map(|r| r[i].marked).all(|b| b);
            if col_complete {
                return col_complete;
            }
        }
        return false;
    }

    fn drawn_num(&mut self, num: u32) {
        for row in self.board.iter_mut() {
            for bn in row.iter_mut() {
                if bn.number == num {
                    bn.marked = true;
                }
            }
        }
    }

    fn score(&self, num: u32) -> u32 {
        let mut score = 0;
        for row in self.board.iter() {
            for bn in row.iter() {
                if !bn.marked {
                    score += bn.number;
                }
            }
        }
        return score * num;
    }
}

pub fn day_04() -> std::io::Result<()> {
    //let mut file = File::open("./resources/day04_small.txt")?;
    let mut file = File::open("./resources/day04.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //println!("contents:\n{}", contents);
    let (drawn_nums, mut boards) = parse_bingo(&contents).unwrap();
    for num in drawn_nums {
        for board in boards.iter_mut() {
            board.drawn_num(num);
            if board.is_bingo() {
                println!("winning board: {}", board);
                println!("Score: {}", board.score(num));
                break;
            }
        }
        if boards.iter().any(|b| b.is_bingo()) {
            break;
        }
    }

    // Let's try again, this time to lose
    let (drawn_nums, mut boards) = parse_bingo(&contents).unwrap();

    for num in drawn_nums {
        // Only keep the non-winning bingo
        let mut boards = boards
            .iter_mut()
            .filter(|b| !b.is_bingo())
            .collect::<Vec<&mut BingoBoard>>();
        for board in boards.iter_mut() {
            board.drawn_num(num);
        }
        if boards.len() == 1 {
            let board = &boards[0];
            println!("losing board: {}", board);
            println!("Score: {}", board.score(num));
            return Ok(());
        }
    }

    return Ok(());
}

fn parse_bingo(input: &str) -> Result<(DrawnNumbers, Vec<BingoBoard>), String> {
    let (drawn_num, boards) = input.split_once("\n\n").unwrap();

    let drawn_num: DrawnNumbers = drawn_num
        .split(',')
        .map(|s| s.trim())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<DrawnNumbers>();

    let boards: Vec<BingoBoard> = boards
        .trim()
        .split("\n\n")
        .map(|s| s.trim())
        .map(BingoBoard::from_string)
        .map(Result::unwrap)
        .collect();

    Ok((drawn_num, boards))
}
