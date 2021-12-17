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

const SIZE: usize = 100;
struct HeightMap {
    map: Vec<u32>,
}

#[derive(Debug)]
struct Location {
    idx: usize,
    height: u32,
}

impl Location {
    fn is_low_point(&self, others: Vec<Location>) -> bool {
        others.iter().all(|x| self.height < x.height)
    }
}

impl HeightMap {
    fn from_lines(lines: Lines<BufReader<File>>) -> Self {
        let map: Vec<_> = lines
            .map(|line| parse_numbers(&line.unwrap()))
            .flat_map(|x| x)
            .collect();
        return Self { map };
    }

    fn get_neighbors(&self, idx: usize) -> Vec<Location> {
        let mut res = Vec::new();
        // not first row
        if idx >= SIZE {
            res.push(Location {
                idx: idx - SIZE,
                height: self.map[idx - SIZE],
            })
        }
        // not first column
        if idx % SIZE != 0 {
            res.push(Location {
                idx: idx - 1,
                height: self.map[idx - 1],
            })
        }

        // not last row
        if idx < SIZE * (SIZE - 1) {
            res.push(Location {
                idx: idx + SIZE,
                height: self.map[idx + SIZE],
            })
        }

        // not last column
        if idx % SIZE != SIZE - 1 {
            res.push(Location {
                idx: idx + 1,
                height: self.map[idx + 1],
            })
        }

        return res;
    }
}

pub(crate) fn day9part1() -> usize {
    let mut lines = read_lines("input9.t").unwrap();
    let m = HeightMap::from_lines(lines);
    let low_points: Vec<_> = m
        .map
        .iter()
        .enumerate()
        .map(|(idx, height)| Location {
            idx,
            height: *height,
        })
        .filter(|loc| loc.is_low_point(m.get_neighbors(loc.idx)))
        .collect();
    println!("{:?}", low_points);
    return low_points.len() + low_points.iter().map(|x| x.height as usize).sum::<usize>();
}
