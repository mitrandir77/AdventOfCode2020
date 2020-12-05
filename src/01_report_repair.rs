#[macro_use]
extern crate scan_rules;

use std::collections::BTreeSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut entries = BTreeSet::new();

    for line in stdin.lock().lines() {
        let_scan!(line.unwrap(); (let entry :i32));
        entries.insert(entry);
    }

    for entry in &entries {
        let matching_entry = 2020 - entry;
        if entries.contains(&matching_entry) {
            println!("{}", entry * matching_entry);
            break;
        }
    }
}
