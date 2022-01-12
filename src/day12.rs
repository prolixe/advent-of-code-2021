use crate::util;
use std::collections::HashSet;

pub fn day_12() -> Result<(), String> {
    //let contents = util::read_file("./resources/day12_small.txt").expect("Could not open file");
    //let contents = util::read_file("./resources/day12_smallest.txt").expect("Could not open file");
    let contents = util::read_file("./resources/day12.txt").expect("Could not open file");

    println!("contents: \n{}", contents);

    let edges: Vec<Edge<String>> = contents
        .trim()
        .split('\n')
        .map(|v| v.try_into().unwrap())
        .collect();

    let nodes: Vec<Node<String>> = contents
        .trim()
        .split('\n')
        .flat_map(|v| v.split('-'))
        .map(|v| v.into())
        .collect::<HashSet<Node<String>>>()
        .into_iter()
        .collect();

    println!("edges {:?}", edges);
    println!("nodes {:?}", nodes);
    let g = Graph::new(nodes, edges);

    let mut seen: HashSet<String> = HashSet::new();
    let result = dfs_part1(&g, "start".to_string(), seen.clone());
    println!("result part 1: {:?}", result);
    let result = dfs_part2(&g, "start".to_string(), seen.clone(), None);
    println!("result part 2: {:?}", result);
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node<T>(T);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Edge<T>(T, T);

struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge<T>>,
}

impl Graph<String> {
    fn new(nodes: Vec<Node<String>>, edges: Vec<Edge<String>>) -> Self {
        Self { nodes, edges }
    }

    fn neighbors(&self, node: Node<String>) -> Vec<Node<String>> {
        let neighbors = self
            .edges
            .iter()
            .filter(|e| e.0 == node.0)
            .map(|e| e.1.clone().into());
        let reverse_neighbors = self
            .edges
            .iter()
            .filter(|e| e.1 == node.0)
            .map(|e| e.0.clone().into());
        neighbors.chain(reverse_neighbors).collect()
    }
}

impl From<&str> for Node<String> {
    fn from(s: &str) -> Self {
        Node(s.to_string())
    }
}
impl From<String> for Node<String> {
    fn from(s: String) -> Self {
        Node(s)
    }
}

impl TryFrom<&str> for Edge<String> {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_once('-') {
            Some((s1, s2)) => Ok(Edge(s1.to_string(), s2.to_string())),
            None => Err("Cannot parse value".to_string()),
        }
    }
}

fn dfs_part1(graph: &Graph<String>, current: String, mut seen: HashSet<String>) -> Option<u32> {
    if current == "end" {
        //println!("reached end: \n seen: {:?}", seen);
        return Some(1);
    }

    // If current is lower, add it to the seen hash.
    //println!("in {}", current);
    if current.to_lowercase() == current {
        seen.insert(current.clone());
    }
    let current_node = Node(current.clone());
    let mut total = 0;
    for n in graph.neighbors(current_node) {
        if seen.contains(&n.0) {
            continue;
        }
        if let Some(count) = dfs_part1(graph, n.0.clone(), seen.clone()) {
            total += count;
        }
    }

    Some(total)
}

fn dfs_part2(
    graph: &Graph<String>,
    current: String,
    mut seen: HashSet<String>,
    mut extra: Option<String>,
) -> Option<u32> {
    if current == "end" {
        //println!("reached end: \n seen: {:?}", seen);
        return Some(1);
    }
    let start = String::from("start");

    // If current is lower, add it to the seen hash.
    //println!("in {}, extra: {:?}", current, extra);
    if current.to_lowercase() == current {
        seen.insert(current.clone());
    }
    let current_node = Node(current.clone());
    let mut total = 0;
    for n in graph.neighbors(current_node) {
        if seen.contains(&n.0) && extra.is_none() {
            if start.eq(&n.0) {
                // Don't visit start twice.
                continue;
            }
            // allow small cave twice only once!
            if let Some(count) = dfs_part1(graph, n.0.clone(), seen.clone()) {
                total += count;
            }
            continue;
        }
        if let Some(count) = dfs_part2(graph, n.0.clone(), seen.clone(), extra.clone()) {
            total += count;
        }
    }

    Some(total)
}
