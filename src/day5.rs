use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Lines, Result},
    path::Path,
};

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct VentLine {
    start: (isize, isize),
    end: (isize, isize),
}

impl VentLine {
    fn from_string(s: &str) -> Option<Self> {
        let x = parse_numbers(s);
        let start = (x[0], x[1]);
        let end = (x[2], x[3]);
        Some(VentLine { start, end })
    }

    fn step(&self) -> (isize, isize) {
        let x_step = ((self.end.0 - self.start.0) as isize).signum();
        let y_step = ((self.end.1 - self.start.1) as isize).signum();
        return (x_step, y_step);
    }
}

pub struct VentLineIterator<'a> {
    vent_line: &'a VentLine,
    index: isize,
    step_x: isize,
    step_y: isize,
    done: bool,
}

impl<'a> Iterator for VentLineIterator<'a> {
    type Item = (isize, isize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let result = (
            self.vent_line.start.0 + (self.step_x * self.index),
            self.vent_line.start.1 + (self.step_y * self.index),
        );
        self.index += 1;
        if result == self.vent_line.end {
            self.done = true;
        }
        Some(result)
    }
}

impl<'a> IntoIterator for &'a VentLine {
    type Item = <VentLineIterator<'a> as Iterator>::Item;

    type IntoIter = VentLineIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let (step_x, step_y) = self.step();
        Self::IntoIter {
            vent_line: self,
            index: 0,
            step_x,
            step_y,
            done: false,
        }
    }
}

struct AllVents {
    vents: Vec<VentLine>,
}

impl AllVents {
    fn parse_vent_lines(mut lines: Lines<BufReader<File>>) -> Self {
        let mut vents = Vec::new();
        while let Some(input_line) = lines.next() {
            if let Some(vent_line) = VentLine::from_string(&input_line.unwrap()) {
                vents.push(vent_line);
            }
        }
        return Self { vents };
    }

    fn all_points_count(&self) -> HashMap<(isize, isize), i32> {
        let mut all_points = HashMap::new();
        for vent in &self.vents {
            for point in vent {
                let e = all_points.entry(point).or_insert(0);
                *e += 1;
            }
        }

        all_points
    }
}

pub(crate) fn day5part1() -> usize {
    let lines = read_lines("input5.t").unwrap();
    let vents = AllVents::parse_vent_lines(lines);
    let point_counts = vents.all_points_count();

    return point_counts.iter().filter(|&(&_, &v)| v > 1).count();
}

fn parse_numbers(s: &str) -> Vec<isize> {
    return s
        .split(&[',', '-', '>', ' '][..])
        .filter(|n| *n != "")
        .map(|n| n.parse().unwrap())
        .collect();
}
