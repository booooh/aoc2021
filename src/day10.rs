use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn is_open_char(c: char) -> bool {
    match c {
        '{' | '[' | '(' | '<' => true,
        _ => false,
    }
}

fn matching_char(c: char) -> char {
    match c {
        '{' => '}',
        '}' => '{',
        '[' => ']',
        ']' => '[',
        '(' => ')',
        ')' => '(',
        '<' => '>',
        '>' => '<',
        _ => ' ',
    }
}

fn is_corrupt(line: &str) -> Option<char> {
    let mut char_stack = Vec::<char>::new();
    for c in line.chars() {
        if is_open_char(c) {
            char_stack.push(c);
        } else {
            if matching_char(c) == char_stack.pop().unwrap() {
                continue;
            } else {
                return Some(c);
            }
        }
    }
    return None;
}

pub(crate) fn day10part1() -> i32 {
    let mut lines = read_lines("input10.t").unwrap();
    // let corrupt_chars = Vec::<char>::new();
    let mut score_map = HashMap::<char, i32>::new();
    score_map.insert(')', 3);
    score_map.insert(']', 57);
    score_map.insert('}', 1197);
    score_map.insert('>', 25137);

    let mut score = 0;
    for line in lines {
        let l = line.unwrap();
        println!("going to check line {}", &l);
        if let Some(err) = is_corrupt(&l) {
            score += score_map.get(&err).unwrap();
        }
    }
    return score;
}
