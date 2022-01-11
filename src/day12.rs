use crate::util;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;

pub fn day_12() -> Result<(), String> {
    let contents = util::read_file("./resources/day12_small.txt").expect("Could not open file");
    //let contents = util::read_file("./resources/day12.txt").expect("Could not open file");

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

impl<T> Graph<T> {
    fn new(nodes: Vec<Node<T>>, edges: Vec<Edge<T>>) -> Self {
        Self { nodes, edges }
    }
}

impl From<&str> for Node<String> {
    fn from(s: &str) -> Self {
        Node(s.to_string())
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
