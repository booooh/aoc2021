use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

fn parse_numbers(s: &str) -> Vec<i32> {
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

struct LanternFish {
    state: i32,
}

impl LanternFish {
    fn next_state(&self) -> Vec<LanternFish> {
        match self.state {
            0 => vec![LanternFish { state: 6 }, LanternFish { state: 8 }],
            _ => vec![LanternFish {
                state: self.state - 1,
            }],
        }
    }
}

fn next_day(school: Vec<LanternFish>) -> Vec<LanternFish> {
    school
        .iter()
        .map(|fish| fish.next_state())
        .flat_map(|s| s)
        .collect()
}

fn by_day(count: i32) -> HashMap<i32, usize> {
    let mut res = HashMap::new();

    // initial seven days, based on the initial state
    for day in 0..7i32 {
        res.insert(day, 1);
    }
    for day in 7..9 {
        res.insert(day, 2);
    }

    for day in 9..count {
        res.insert(
            day,
            res.get(&(day - 7)).unwrap() + res.get(&(day - 9)).unwrap(),
        );
    }

    return res;
}

pub(crate) fn day6part1() -> usize {
    let mut lines = read_lines("input6.t").unwrap();
    let mut fish: Vec<_> = parse_numbers(&lines.next().unwrap().unwrap())
        .iter()
        .map(|&state| LanternFish { state })
        .collect();
    for _ in 0..80 {
        fish = next_day(fish);
    }
    return fish.len();
}

pub(crate) fn day6part2() -> usize {
    let mut lines = read_lines("input6.t").unwrap();
    let mut fish = parse_numbers(&lines.next().unwrap().unwrap());
    let fish_by_day = by_day(256 + 7);
    let total_count = fish
        .iter()
        .map(|state| fish_by_day.get(&(256 + (6 - state))).unwrap())
        .sum();

    println!("{:?}", total_count);
    return total_count;
}
