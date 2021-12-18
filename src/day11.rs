use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
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

const FLASH_LEVEL: i32 = 9;

#[derive(Debug)]
struct Octopus {
    energy_level: i32,
    num_flashes: usize,
    flashed_this_step: bool,
}

impl Octopus {
    fn increase_energy(&mut self) -> bool {
        // do not increase energy, if already flashed this step
        if self.flashed_this_step {
            return false;
        }

        if self.energy_level < FLASH_LEVEL {
            self.energy_level += 1;
        } else if self.energy_level == FLASH_LEVEL {
            self.energy_level = 0;
            self.num_flashes += 1;
            self.flashed_this_step = true;
            return true;
        }
        return false;
    }

    fn step(&mut self) -> bool {
        self.flashed_this_step = false;
        return self.increase_energy();
    }
}

#[derive(Debug)]
struct OctopusGarden {
    octopuses: Vec<Octopus>,
}

impl Display for OctopusGarden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = (self.octopuses.len() as f32).sqrt() as usize;
        for (idx, oct) in self.octopuses.iter().enumerate() {
            f.write_fmt(format_args!("{}", oct.energy_level))?;
            if idx % size == size - 1 {
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}

impl OctopusGarden {
    fn step(&mut self) {
        // step each of the octopuses, and then flash neighbors as needed
        // let mut flashed: HashSet<usize> = self.octopuses.iter_mut().enumerate().filter(|(idx, oct)| oct.step()).map() collect();
        let mut flashed: VecDeque<usize> = VecDeque::new();
        for (idx, oct) in self.octopuses.iter_mut().enumerate() {
            if oct.step() {
                flashed.push_back(idx);
            }
        }

        while flashed.len() > 0 {
            // expand via BFS from each of the nodes
            let oct_idx = flashed.pop_front().unwrap();
            for n in self.get_neighbors(oct_idx) {
                if self.octopuses[n].increase_energy() {
                    flashed.push_back(n);
                }
            }
        }
    }

    fn get_neighbors(&self, idx: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let size = (self.octopuses.len() as f32).sqrt() as usize;
        let is_left_most = idx % size == 0;
        let is_right_most = idx % size == size - 1;
        let is_top = idx < size;
        let is_bottom = idx >= size * (size - 1);

        // not first row
        if !is_top {
            res.push(idx - size);
            // add diagonals
            if !is_left_most {
                res.push((idx - size) - 1);
            }

            if !is_right_most {
                res.push((idx - size) + 1);
            }
        }

        // not last row
        if !is_bottom {
            res.push(idx + size);
            // add diagonals
            if !is_left_most {
                res.push((idx + size) - 1);
            }

            if !is_right_most {
                res.push((idx + size) + 1);
            }
        }

        // not first column
        if !is_left_most {
            res.push(idx - 1);
        }

        // not last column
        if !is_right_most {
            res.push(idx + 1);
        }

        return res;
    }
}

pub(crate) fn day11part1() -> usize {
    let mut lines = read_lines("input11.t").unwrap();
    let octopuses = lines
        .map(|l| l.unwrap())
        .map(|l| parse_numbers(&l))
        .flat_map(|f| f.into_iter())
        .map(|energy_level| Octopus {
            energy_level,
            flashed_this_step: false,
            num_flashes: 0,
        })
        .collect();
    let mut garden = OctopusGarden { octopuses };
    println!("{}", garden);
    for _ in 0..100 {
        garden.step();
        println!("{}", garden);
    }

    let res = garden.octopuses.iter().map(|oct| oct.num_flashes).sum();

    return res;
}

pub(crate) fn day11part2() -> usize {
    let mut lines = read_lines("input11.t").unwrap();
    let octopuses = lines
        .map(|l| l.unwrap())
        .map(|l| parse_numbers(&l))
        .flat_map(|f| f.into_iter())
        .map(|energy_level| Octopus {
            energy_level,
            flashed_this_step: false,
            num_flashes: 0,
        })
        .collect();
    let mut garden = OctopusGarden { octopuses };
    println!("{}", garden);
    let mut count = 0;
    loop {
        garden.step();
        count += 1;
        println!("{}\n{}", count, garden);
        if garden.octopuses.iter().all(|o| o.flashed_this_step) {
            break;
        }
    }

    return count;
}
