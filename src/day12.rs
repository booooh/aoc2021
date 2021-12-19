use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
    thread::current,
};

fn parse_line(s: &str) -> (String, String) {
    let mut pair = s.split('-').collect::<Vec<&str>>();
    (pair[0].to_string(), pair[1].to_string())
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[derive(Debug)]
struct Graph {
    nodes: HashSet<String>,
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    fn from_lines(lines: Lines<BufReader<File>>) -> Self {
        let mut graph = Graph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        };
        for line in lines {
            let (a, b) = parse_line(&line.unwrap());
            graph.nodes.insert(a.clone());
            graph.nodes.insert(b.clone());
            let edges_a = graph.edges.entry(a.clone()).or_insert(vec![]);
            if b != "start" {
                edges_a.push(b.clone());
            }
            let edges_b = graph.edges.entry(b).or_insert(vec![]);
            if a != "start" {
                edges_b.push(a);
            }
        }

        // remove edges starting at 'end' node
        graph.edges.insert("end".into(), Vec::new());

        return graph;
    }

    // fn backtrack(&self, mut path: Vec<String>) -> (Vec<String>, String) {
    //     // 1. remove the latest item in the path
    //     let curr_last = path.pop().unwrap();

    //     // take a new sibling for the last element
    //     let prev_last = path.last().unwrap();
    //     let possible_neighbors = &self.edges[prev_last];
    //     let new_index = possible_neighbors
    //         .iter()
    //         .position(|x| x == &curr_last)
    //         .unwrap()
    //         + 1;
    //     if new_index == possible_neighbors.len() {
    //         // this was the last sibling, remove it
    //     }
    // }

    fn all_paths(&self, start: String, end: String, day2: bool) -> HashSet<Vec<String>> {
        let mut visited = Vec::<String>::new();
        visited.push(start);
        let mut paths = HashSet::<Vec<String>>::new();
        let mut next_index = 0;

        // assumes there are no loops - so just add to the path if it's possible
        loop {
            // try to extend the path, find the next sibling after the last one used
            let current = visited.last().unwrap().to_owned();
            let possible_extensions = self.edges.get(&current).unwrap();
            if possible_extensions.len() > next_index {
                // check if valid:
                let ext = &possible_extensions[next_index];
                if &ext.to_uppercase() == ext || !visited.contains(ext) {
                    visited.push(ext.to_string());
                    next_index = 0;
                } else {
                    next_index += 1;
                }
            } else {
                // if this is a good path, record it
                if current == end {
                    // println!("found end, going to save the path");
                    paths.insert(visited.clone());
                }

                //now backtrack, if possible - if not - return:
                let dropped_node = visited.pop().unwrap();
                if visited.len() == 0 {
                    return paths;
                }

                // index of the next element
                let last_node = visited.last().unwrap();
                next_index = self
                    .edges
                    .get(last_node)
                    .unwrap()
                    .iter()
                    .position(|x| x == &dropped_node)
                    .unwrap()
                    + 1;
            }
        }
    }
}

pub(crate) fn day12part1() -> usize {
    let mut lines = read_lines("input12.t").unwrap();
    let g = Graph::from_lines(lines);
    println!("{:?}", g);
    let paths = g.all_paths("start".to_string(), "end".to_string(), false);
    println!("{:?}", paths);
    return paths.len();
}

pub(crate) fn day12part2() -> usize {
    let mut lines = read_lines("input12.t").unwrap();
    return 0;
}
