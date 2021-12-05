use std::fmt::Debug;
use std::io::BufRead;
use std::{fs::File, io, path::Path};

fn main() {
    // println!("day1part1 {}", day1part1());
    // println!("day1part2 {}", day1part2());

    // println!("day2part2 {:?}", day2part2());
    println!("day3part1 {:?}", day3part1());
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

fn day3part1() -> (i32, i32) {
    let lines = read_lines("input3.t").unwrap();
    println!("read lines");
    let binary_numbers: Vec<_> = lines.map(|l| parse_status_bits(l.unwrap())).collect();
    println!("got binary numbers");
    let bit_values = data_by_bit(binary_numbers);
    println!("got bit values");
    let zero_counts: Vec<usize> = bit_values
        .iter()
        .map(|values| count_value(&0, values))
        .collect();
    let most_common_bits: Vec<u8> = zero_counts
        .iter()
        .zip(bit_values.iter())
        .map(|(num_zero, all)| if *num_zero > all.len() / 2 { 0u8 } else { 1u8 })
        .collect();
    let least_common_bits: Vec<u8> = most_common_bits
        .iter()
        .map(|b| if *b == 0 { 1u8 } else { 0u8 })
        .collect();
    let gamma = bit_vector_to_number(&most_common_bits);
    let epsilon = bit_vector_to_number(&least_common_bits);
    return (gamma, epsilon);
}

fn parse_status_bits(s: String) -> Vec<u8> {
    s.chars().map(|c| c.to_digit(2).unwrap() as u8).collect()
}

fn data_by_bit(binary_numbers: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let num_bits = binary_numbers[0].len();

    // this will return a vector with num_bits entries, each entry will have all the data points for that bit
    let mut res = vec![Vec::<u8>::new(); num_bits];
    for status_bits in binary_numbers {
        for (idx, bit) in status_bits.iter().enumerate() {
            res[idx].push(*bit);
        }
    }
    return res;
}

fn count_value(value: &u8, data: &Vec<u8>) -> usize {
    data.iter().filter(|x| *x == value).count()
}

fn bit_vector_to_number(bit_vector: &Vec<u8>) -> i32 {
    let string_bits = bit_vector
        .iter()
        .map(|bit| bit.to_string())
        .collect::<String>();
    println!("{}", string_bits);
    let val = i32::from_str_radix(&string_bits, 2).unwrap();
    println!("as int {}", val);
    return val;
}
