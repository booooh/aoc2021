use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

fn parse_digits(s: String) -> Vec<String> {
    return s
        .split(&['|', ' '][..])
        .filter(|n| *n != "")
        .map(|x| x.to_owned())
        .collect();
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

struct Entry {
    signal_patterns: Vec<String>,
    output_value: Vec<String>,
    map_num_to_pattern: HashMap<i32, String>,
    map_pattern_to_num: HashMap<String, i32>,
}

fn sorted_pattern(pattern: &String) -> String {
    let mut pc = pattern.chars().collect::<Vec<_>>();
    pc.sort();
    pc.iter().collect()
}

impl Entry {
    fn found_pattern(&mut self, pattern: &String, num: i32) {
        self.map_num_to_pattern.insert(num, pattern.to_owned());
        self.map_pattern_to_num.insert(pattern.to_owned(), num);
    }

    fn from_strings(line: Vec<String>) -> Self {
        let mut res = Self {
            signal_patterns: line[0..10].iter().map(|f| sorted_pattern(f)).collect(),
            output_value: line[10..14].iter().map(|f| sorted_pattern(f)).collect(),
            map_num_to_pattern: HashMap::new(),
            map_pattern_to_num: HashMap::new(),
        };

        // sort the patterns and output values

        // populate the easy ones
        let patterns = res.signal_patterns.clone();
        for p in &patterns {
            match p.len() {
                2 => {
                    res.found_pattern(p, 1);
                }
                4 => {
                    res.found_pattern(p, 4);
                }
                3 => {
                    res.found_pattern(p, 7);
                }
                7 => {
                    res.found_pattern(p, 8);
                }
                _ => {}
            }
        }
        let one_chars: HashSet<_> = res.map_num_to_pattern.get(&1).unwrap().chars().collect();
        let four_chars: HashSet<_> = res.map_num_to_pattern.get(&4).unwrap().chars().collect();

        // we can now populate 2, 3, 6
        for p in &patterns {
            let c: HashSet<_> = p.chars().collect();
            match p.len() {
                6 => {
                    if !c.is_superset(&one_chars) {
                        res.found_pattern(&p, 6);
                    }
                }
                5 => {
                    if c.union(&four_chars).collect::<HashSet<_>>().len() == 7 {
                        res.found_pattern(&p, 2);
                    } else if c.is_superset(&one_chars) {
                        res.found_pattern(&p, 3);
                    }
                }
                _ => {}
            }
        }

        let three_chars: HashSet<_> = res.map_num_to_pattern.get(&3).unwrap().chars().collect();
        let four_and_three: HashSet<_> =
            HashSet::from_iter(four_chars.union(&three_chars).map(|&x| x));

        // we can now populate 0,5,9
        for p in &patterns {
            if res.map_pattern_to_num.contains_key(p) {
                continue;
            }

            let c: HashSet<_> = p.chars().collect();
            match p.len() {
                5 => {
                    res.found_pattern(p, 5);
                }
                6 => {
                    println!("{:?}, {:?}", c, four_and_three);
                    if c == four_and_three {
                        println!("found 9");
                        res.found_pattern(p, 9);
                    } else {
                        res.found_pattern(p, 0);
                    }
                }
                _ => {}
            }
        }

        return res;
    }

    fn get_output_number(&self) -> i32 {
        let as_str: String = self.output_value.iter().fold("".to_owned(), |acc, x| {
            acc + &self.map_pattern_to_num.get(x).unwrap().to_string()
        });
        as_str.parse().unwrap()
    }
}

fn match_pattern(pattern: &String) -> String {
    let out = match pattern.len() {
        2 => "1",
        4 => "4",
        3 => "7",
        7 => "8",
        _ => "",
    };

    return out.to_owned();
}
/**
 *
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

1,4,7,8
has 6 segments, and doesn't have all segments of "1" ==> 6
has 5 segments and with union of "4" has 7 segements => 2
has 5 segments and has all segments of "1" => 3

has 5 segments and is not 2 or 3 => 5
union of 4 and 3 => 9
has 6 segments and is not 6 or 9 => 0

 */

pub(crate) fn day8part1() -> usize {
    let mut lines = read_lines("input8.t").unwrap();
    let input: Vec<_> = lines
        .map(|line| Entry::from_strings(parse_digits(line.unwrap())).output_value)
        .flat_map(|x| x)
        .filter(|out_value| match_pattern(out_value) != "")
        .collect();
    println!("{:?}", input);
    return input.len();
}

pub(crate) fn day8part2() -> i32 {
    let mut lines = read_lines("input8.t").unwrap();
    let mut input: Vec<_> = lines
        .map(|line| Entry::from_strings(parse_digits(line.unwrap())).get_output_number())
        .collect();
    println!("{:?}", input);
    return input.iter().sum();
}
