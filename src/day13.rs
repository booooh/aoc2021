use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Display, Write},
    fs::File,
    io::{self, BufRead, BufReader, Lines, Read},
    num::ParseIntError,
    ops::Add,
    path::Path,
    thread::current,
};

fn parse_line(s: &str) -> Result<(i32, i32), ParseIntError> {
    let mut pair = s.split(',').collect::<Vec<&str>>();
    Ok((pair[0].to_string().parse()?, pair[1].to_string().parse()?))
}

fn read_buffer<P>(filename: P) -> io::Result<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}

#[derive(Debug)]
struct TransparentPaper {
    dots: HashSet<(i32, i32)>,
    folds: VecDeque<(char, i32)>,
}

impl TransparentPaper {
    fn from_lines(buf: &mut BufReader<File>) -> Self {
        let mut paper = TransparentPaper {
            dots: HashSet::new(),
            folds: VecDeque::new(),
        };

        for line in buf.lines() {
            if let Ok(coord) = parse_line(&line.unwrap()) {
                paper.dots.insert(coord);
            } else {
                break; // this skips the empty line between dots and folds
            }
        }

        for line in buf.lines() {
            let unwrapped = line.unwrap();
            let mut parts = unwrapped.split(&[' ', '='][..]).rev();
            let index = parts.next().unwrap().parse::<i32>().unwrap();
            let direction = parts.next().unwrap().chars().next().unwrap();
            paper.folds.push_back((direction, index));
        }

        return paper;
    }

    fn fold_once(&mut self) {
        let f = self.folds.pop_front().unwrap();

        let final_fold: Box<dyn Fn((i32, i32)) -> (i32, i32)> = match f.0 {
            'x' => Box::new(|c| (f.1 - (c.0 - f.1).abs(), c.1)),
            'y' => Box::new(|c| (c.0, f.1 - (c.1 - f.1).abs())),
            _ => return,
        };

        self.dots = self.dots.iter().map(|&c| final_fold(c)).collect();
    }

    fn fold_all(&mut self) {
        while self.folds.len() > 0 {
            self.fold_once();
        }
    }
}

impl Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.dots.iter().map(|(x, y)| x).max().unwrap();
        let max_y = self.dots.iter().map(|(x, y)| y).max().unwrap();
        for y in 0..max_y + 1 {
            for x in 0..max_x + 1 {
                if self.dots.contains(&(x, y)) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

pub(crate) fn day13part1() -> usize {
    let mut buf = read_buffer("input13.t").unwrap();
    let mut p = TransparentPaper::from_lines(buf.by_ref());
    p.fold_all();
    println!("{}", p);
    return p.dots.len();
}
