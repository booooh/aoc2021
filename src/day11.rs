use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

fn parse_numbers(s: &str) -> Vec<i32> {
    return s.chars().map(|n| n.to_digit(10).unwrap() as i32).collect();
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub(crate) fn day11part1() -> usize {
    let mut lines = read_lines("input11.t").unwrap();
    return 0;
}

pub(crate) fn day11part2() -> usize {
    let lines = read_lines("input9.t").unwrap();
    return 0;
}
