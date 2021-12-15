use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

fn parse_numbers(s: &str) -> Vec<isize> {
    return s
        .split(&[','][..])
        .filter(|n| *n != "")
        .map(|n| n.parse().unwrap())
        .collect();
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub(crate) fn day7part1() -> isize {
    let mut lines = read_lines("input7.t").unwrap();
    let mut locations = parse_numbers(&lines.next().unwrap().unwrap());
    println!("{}", locations.len());
    locations.sort();
    println!("{}", locations[500]);
    let m1 = locations.iter().map(|&x| (x - locations[500]).abs()).sum();
    let m2: isize = locations.iter().map(|&x| (x - locations[501]).abs()).sum();

    println!("{} {}", m1, m2);

    return m1;
}

fn cost_day_2(location: isize, dest: isize) -> isize {
    let abs_dist = (location - dest).abs();
    return ((abs_dist + 1) * abs_dist) / 2;
}

pub(crate) fn day7part2() -> isize {
    let mut lines = read_lines("input7.t").unwrap();
    let mut locations = parse_numbers(&lines.next().unwrap().unwrap());
    let len = locations.len();
    println!("{} {}", locations.iter().sum::<isize>(), len);
    let avg1 = (locations.iter().sum::<isize>() as f32 / len as f32).floor() as isize;
    let avg2 = (locations.iter().sum::<isize>() as f32 / len as f32).ceil() as isize;
    println!("{} {}", avg1, avg2);
    let m1: isize = locations.iter().map(|&x| cost_day_2(x, avg1)).sum();
    let m2: isize = locations.iter().map(|&x| cost_day_2(x, avg2)).sum();
    println!("{} {}", m1, m2);
    return m1.min(m2);
}
