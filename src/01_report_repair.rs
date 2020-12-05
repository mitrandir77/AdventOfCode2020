#[macro_use]
extern crate scan_rules;

use multiset::HashMultiSet;

use std::io::{self, BufRead};

fn n_entries_that_sum_to_x(entries: &mut HashMultiSet<i32>, n: i32, x: i32) -> Option<Vec<i32>> {
    if n == 1 {
        if entries.contains(&x) {
            return Some(vec![x]);
        }
    }
    else {
        let entries_copy = entries.clone();
        for entry in entries_copy.iter() {
            let complement = x - entry;
            entries.remove(entry);
            if let Some(mut ret) = n_entries_that_sum_to_x(entries, n - 1, complement) {
                ret.push(*entry);
                return Some(ret);
            }
            entries.insert(*entry);
        }

    }
    return None;
}

fn main() {
    let stdin = io::stdin();

    let mut entries = HashMultiSet::new();

    for line in stdin.lock().lines() {
        let_scan!(line.unwrap(); (let entry :i32));
        entries.insert(entry);
    }

    if let Some(ret) = n_entries_that_sum_to_x(&mut entries, 2, 2020) {
        println!("Product of two entries that sum up to 2020 is {}", ret[0] * ret[1]);
    } else {
        println!("No two entries sum to 2020");
    }
}
