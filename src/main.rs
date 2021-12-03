use std::fmt::Debug;
use std::io::BufRead;
use std::{fs::File, io, path::Path};

fn main() {
    println!("day1part1 {}", day1part1());
    println!("day1part2 {}", day1part2());

    println!("day2part1 {:?}", day2part1());
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

fn day2part1() -> Location {
    let lines = read_lines("input2.t").unwrap();
    let mut location = Location {
        horizontal: 0,
        depth: 0,
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
}

impl Location {
    fn process_command(mut self, command: &Command) -> Self {
        match command.direction {
            Direction::Forward => self.horizontal = self.horizontal + command.units,
            Direction::Down => self.depth = self.depth + command.units,
            Direction::Up => self.depth = self.depth - command.units,
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
