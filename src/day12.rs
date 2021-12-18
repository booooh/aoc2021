use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
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
}

pub(crate) fn day12part1() -> usize {
    let mut lines = read_lines("input12.t").unwrap();
    let g = Graph::from_lines(lines);
    println!("{:?}", g);
    return 0;
}

pub(crate) fn day12part2() -> usize {
    let mut lines = read_lines("input12.t").unwrap();
    return 0;
}
