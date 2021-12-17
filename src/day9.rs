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

const SIZE: usize = 100;
struct HeightMap {
    map: Vec<i32>,
}

#[derive(Debug)]
struct Location {
    idx: usize,
    height: i32,
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

    fn mark_basins(&mut self) -> i32 {
        // iterate through the heights - everything that isn't a nine becomes 0, nine becomes -1
        for height in self.map.iter_mut() {
            if *height == 9 {
                *height = -1;
            } else {
                *height = 0;
            }
        }

        // now loop through all items that are 0s, and find the connected component
        let mut basin_num = 1;
        for idx in 0..self.map.len() {
            if self.map[idx] == 0 {
                self.mark_neighbors_in_basin(idx, basin_num);
                basin_num += 1;
            }
        }

        return basin_num;
    }

    fn mark_neighbors_in_basin(&mut self, idx: usize, basin_num: i32) {
        let mut basin_neighbors = VecDeque::new();
        basin_neighbors.push_back(idx);
        while let Some(cur_idx) = basin_neighbors.pop_front() {
            // find new neighbors that haven't been marked
            for idx in self
                .get_neighbors(cur_idx)
                .iter()
                .filter(|&loc| loc.height == 0)
                .map(|loc| loc.idx)
            {
                // mark the neighbors
                basin_neighbors.push_back(idx);
                self.map[idx] = basin_num;
            }
        }
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

pub(crate) fn day9part2() -> usize {
    let lines = read_lines("input9.t").unwrap();
    let mut m = HeightMap::from_lines(lines);
    let num_basins = m.mark_basins();
    println!("Found {} basins", num_basins);
    let mut basin_sizes = Vec::<usize>::new();
    for b in 1..num_basins {
        basin_sizes.push(m.map.iter().filter(|&x| *x == b).count());
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    return basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
}
