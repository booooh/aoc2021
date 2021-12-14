use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines, Result},
    path::Path,
};

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) fn day4part1() -> (usize, usize) {
    let mut lines = read_lines("input4.t").unwrap();
    let mut game = BingoGame::from_lines(&mut lines);
    game.play()
}

pub(crate) fn day4part2() -> (usize, usize) {
    let mut lines = read_lines("input4.t").unwrap();
    let mut game = BingoGame::from_lines(&mut lines);
    game.play_to_lose()
}

const BOARD_SIZE: usize = 5;

struct BingoCard {
    board: Vec<(u8, bool)>,
}

struct BingoGame {
    boards: Vec<BingoCard>,
    numbers: Vec<u8>,
}

impl BingoCard {
    fn from_lines(lines: &mut Lines<BufReader<File>>) -> Option<Self> {
        // skip the empt line
        lines.next();
        let mut numbers = Vec::<u8>::new();
        for _ in 0..BOARD_SIZE {
            if let Some(line) = lines.next() {
                numbers.extend(parse_numbers(line.unwrap()));
            } else {
                return None;
            }
        }
        let board: Vec<(u8, bool)> = numbers.iter().map(|num| (*num, false)).collect();
        println!("created a board with {} numbers", board.len());
        return Some(BingoCard { board });
    }

    fn mark(&mut self, number: u8) {
        if let Some(item) = self.board.iter_mut().find(|(value, _)| *value == number) {
            item.1 = true;
        }
    }

    fn score(&self) -> usize {
        return self
            .board
            .iter()
            .filter(|(val, mark)| *mark == false)
            .map(|(val, mark)| *val as usize)
            .sum();
    }

    fn has_won(&self) -> bool {
        // check rows
        for row in 0..BOARD_SIZE {
            let mut all_cols = true;
            for col in 0..BOARD_SIZE {
                let idx = row * BOARD_SIZE + col;
                all_cols = all_cols && self.board[idx].1 == true;
            }

            if all_cols == true {
                return true;
            }
        }
        // check columns
        for col in 0..BOARD_SIZE {
            let mut all_rows = true;
            for row in 0..BOARD_SIZE {
                let idx = row * BOARD_SIZE + col;
                all_rows = all_rows && self.board[idx].1 == true;
            }
            if all_rows == true {
                return true;
            }
        }
        return false;
    }
}

impl BingoGame {
    fn from_lines(lines: &mut Lines<BufReader<File>>) -> Self {
        // read number input
        let input_line = lines.next().unwrap().unwrap();
        let mut game = BingoGame {
            boards: Vec::<BingoCard>::new(),
            numbers: parse_numbers(input_line),
        };
        while let Some(board) = BingoCard::from_lines(lines) {
            game.boards.push(board);
        }

        return game;
    }

    fn play(&mut self) -> (usize, usize) {
        for num in &self.numbers {
            for board in &mut self.boards {
                board.mark(*num);
                if board.has_won() {
                    return (*num as usize, board.score());
                }
            }
        }

        return (0, 0);
    }

    fn play_to_lose(&mut self) -> (usize, usize) {
        for num in &self.numbers {
            for board in &mut self.boards {
                board.mark(*num);
            }
            if self.boards.len() == 1 && self.boards[0].has_won() {
                return (*num as usize, self.boards[0].score());
            }
            self.boards.retain(|board| !board.has_won());
        }

        return (0, 0);
    }
}

fn parse_numbers(s: String) -> Vec<u8> {
    println!("{}", s);
    return s
        .split(&[',', ' '][..])
        .filter(|n| *n != "")
        .map(|n| n.parse().unwrap())
        .collect();
}
