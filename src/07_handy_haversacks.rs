#[macro_use]
extern crate scan_rules;
#[macro_use]
extern crate maplit;

use scan_rules::{
    scanner::{re_str, Word},
    ScanError,
};

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{self, BufRead};

// Inverts graph edges.
fn invert_graph<T: Eq + Hash + Copy>(graph: &HashMap<T, Vec<T>>) -> HashMap<T, Vec<T>> {
    let mut inverted_graph: HashMap<T, Vec<T>> = HashMap::new();
    for (src, dsts) in graph.iter() {
        for dst in dsts {
            let mut new_edge = inverted_graph.remove(dst).unwrap_or_default();
            new_edge.push(*src);
            inverted_graph.insert(*dst, new_edge);
        }
    }
    inverted_graph
}

// Returns set of visited nodes
fn dfs<T: Eq + Hash + Copy>(graph: &HashMap<T, Vec<T>>, start: T) -> HashSet<T> {
    let mut stack = vec![start];
    let mut visited = hashset! {start};

    while let Some(node) = stack.pop() {
        let edges = graph.get(&node);

        if let Some(edges) = edges {
            for edge in edges {
                if !visited.contains(edge) {
                    visited.insert(*edge);
                    stack.push(*edge);
                }
            }
        }
    }
    visited
}

fn main() -> Result<(), ScanError> {
    let stdin = io::stdin();
    let mut graph: HashMap<(&str, &str), Vec<(&str, &str)>> = HashMap::new();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let iter = lines.iter();
    for line in iter {
        (scan! {line;
            (let sub_adj: Word, let sub_clr: Word , " bags contain ", [let num: i32, let adj: Word, let clr: Word, let bags <| re_str(r"bag(s)?")], +, ".") => {
                graph.insert((sub_adj, sub_clr), adj.into_iter().zip(clr).collect());
            },
            (let adj: Word, let clr: Word , " bags contain no other bags.") => (),
        })?;
    }

    let inverted_graph = invert_graph(&graph);
    let mut visited = dfs(&inverted_graph, ("shiny", "gold"));
    visited.remove(&("shiny", "gold"));

    println!(
        "Shiny gold bags can be contained by {} bag colors",
        visited.len()
    );

    Ok(())
}
