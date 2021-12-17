use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

fn parse_numbers(s: &str) -> Vec<u32> {
    return s.chars().map(|n| n.to_digit(10).unwrap()).collect();
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub(crate) fn day9part1() -> usize {
    let mut lines = read_lines("input9.t").unwrap();
    let input: Vec<_> = lines
        .map(|line| parse_numbers(&line.unwrap()))
        .flat_map(|x| x)
        .collect();
    println!("{:?}", input);
    return input.len();
}
