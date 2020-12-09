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

type Graph<T> = HashMap<T, Vec<(i32, T)>>;
// Inverts graph edges.
fn invert_graph<T: Eq + Hash + Copy>(graph: &Graph<T>) -> Graph<T> {
    let mut inverted_graph: Graph<T> = HashMap::new();
    for (src, dsts) in graph.iter() {
        for (cnt, dst) in dsts {
            let mut new_edge = inverted_graph.remove(dst).unwrap_or_default();
            new_edge.push((*cnt, *src));
            inverted_graph.insert(*dst, new_edge);
        }
    }
    inverted_graph
}

// Returns set of visited nodes
fn dfs<T: Eq + Hash + Copy>(graph: &Graph<T>, start: T) -> HashSet<T> {
    let mut stack = vec![start];
    let mut visited = hashset! {start};

    while let Some(node) = stack.pop() {
        let edges = graph.get(&node);

        if let Some(edges) = edges {
            for (_cnt, edge) in edges {
                if !visited.contains(edge) {
                    visited.insert(*edge);
                    stack.push(*edge);
                }
            }
        }
    }
    visited
}

fn count_bags<T: Eq + Hash + Copy>(graph: &Graph<T>, start: T, cache: &mut HashMap<T, i32>) -> i32 {
    let edges = graph.get(&start);
    let mut result = 1;

    if let Some(edges) = edges {
        for (cnt, edge) in edges {
            if !cache.contains_key(edge) {
                let edge_result = count_bags(graph, *edge, cache);
                cache.insert(*edge, edge_result);
            }
            result += cnt * cache.get(edge).unwrap_or(&0);
        }
    }
    result
}

fn main() -> Result<(), ScanError> {
    let stdin = io::stdin();
    let mut graph: Graph<(&str, &str)> = HashMap::new();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let iter = lines.iter();
    for line in iter {
        (scan! {line;
            (let sub_adj: Word, let sub_clr: Word , " bags contain ", [let num: i32, let adj: Word, let clr: Word, let bags <| re_str(r"bag(s)?")], +, ".") => {
                graph.insert((sub_adj, sub_clr), num.into_iter().zip(adj.into_iter().zip(clr)).collect());
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

    let mut cache = HashMap::new();
    let count = count_bags(&graph, ("shiny", "gold"), &mut cache);
    println!("Shiny gold bags can contain {} bags", count - 1);

    Ok(())
}
