mod day10;
mod day11;
mod day12;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use day3::day3part1;
use std::fmt::Debug;
use std::io::BufRead;
use std::{fs::File, io, path::Path};

use crate::day10::{day10part1, day10part2};
use crate::day11::{day11part1, day11part2};
use crate::day12::{day12part1, day12part2};
use crate::day3::day3part2;
use crate::day4::{day4part1, day4part2};
use crate::day5::day5part1;
use crate::day6::{day6part1, day6part2};
use crate::day7::{day7part1, day7part2};
use crate::day8::{day8part1, day8part2};
use crate::day9::{day9part1, day9part2};

fn main() {
    // println!("day1part1 {}", day1part1());
    // println!("day1part2 {}", day1part2());

    // println!("day2part2 {:?}", day2part2());
    // println!("day3part1 {:?}", day3part1());
    // println!("day3part2 {:?}", day3part2());
    // println!("day4part1 {:?}", day4part1());
    // println!("day4part2 {:?}", day4part2());
    // println!("day5part1 {:?}", day5part1());
    // println!("day6part1 {:?}", day6part1());
    // println!("day6part2 {:?}", day6part2());
    // println!("day7part1 {:?}", day7part1());
    // println!("day7part2 {:?}", day7part2());
    // println!("day8part1 {:?}", day8part1());
    // println!("day8part2 {:?}", day8part2());
    // println!("day9part1 {:?}", day9part1());
    // println!("day9part2 {:?}", day9part2());
    // println!("day10part1 {:?}", day10part1());
    // println!("day10part2 {:?}", day10part2());
    // println!("day11part1 {:?}", day11part1());
    // println!("day11part2 {:?}", day11part2());
    // println!("day12part1 {:?}", day12part1());
    println!("day12part2 {:?}", day12part2());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day1part1() -> i32 {
    let x: Vec<i32> = read_lines("input1.t")
        .unwrap()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let mut i1 = x.iter();
    i1.next();
    return i1.zip(x.iter()).map(|m| (m.0 > m.1) as i32).sum();
}

fn day1part2() -> i32 {
    let x: Vec<i32> = read_lines("input1.t")
        .unwrap()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let window_size = 3;
    let num_extra = window_size - 1;

    // create a vector of slices, each the size of the desired window
    let windows = (0..x.len() - num_extra)
        .map(|idx| &x[idx..(idx + window_size)])
        .collect::<Vec<&[i32]>>();

    // sum each window
    let sums: Vec<i32> = windows.iter().map(|w| w.iter().sum()).collect();
    // println!(
    //     "number of measurements {} number of sums {}",
    //     x.len(),
    //     sums.len()
    // );

    // println!("{:?}", &x[0..12]);
    // println!("{:?}", &sums[0..10]);

    let mut i1 = sums.iter();
    i1.next();
    return i1.zip(sums.iter()).map(|m| (m.0 > m.1) as i32).sum();
}

fn day2part2() -> Location {
    let lines = read_lines("input2.t").unwrap();
    let mut location = Location {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    let commands: Vec<_> = lines.map(|l| parse_command(l.unwrap()).unwrap()).collect();
    for command in commands {
        location = location.process_command(&command);
    }
    return location;
}

enum Direction {
    Forward,
    Down,
    Up,
}

struct Command {
    units: i32,
    direction: Direction,
}

#[derive(Debug)]
struct Location {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Location {
    fn process_command(mut self, command: &Command) -> Self {
        match command.direction {
            Direction::Forward => {
                self.horizontal = self.horizontal + command.units;
                self.depth = self.depth + self.aim * command.units
            }
            Direction::Down => self.aim = self.aim + command.units,
            Direction::Up => self.aim = self.aim - command.units,
        }
        return self;
    }
}

fn parse_command(s: String) -> Result<Command, ()> {
    let x: Vec<_> = s.split(" ").collect();
    let direction = match x[0] {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => return Err(()),
    };

    let units = x[1].parse().unwrap();
    Ok(Command { direction, units })
}
