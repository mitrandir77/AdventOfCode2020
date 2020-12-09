#[macro_use]
extern crate scan_rules;

use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::io::{self, BufRead};

fn find_invalid_number(numbers: &[i64]) -> Option<i64> {
    // Those two datastructures always store the same elements.
    let mut buffer_queue: VecDeque<i64> = VecDeque::new();
    let mut buffer_set = BTreeSet::new();
    for n in numbers {
        let mut found = false;
        if buffer_queue.len() == 25 {
            for candidate in buffer_queue.iter() {
                if buffer_set.contains(&(n - candidate)) {
                    found = true;
                    break;
                }
            }
            if !found {
                return Some(*n);
            }
            let x = buffer_queue.pop_front().unwrap();
            buffer_set.remove(&x);
        }
        buffer_set.insert(*n);
        buffer_queue.push_back(*n);
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut numbers = Vec::new();
    for line in stdin.lock().lines() {
        let_scan!(line.unwrap(); (let n: i64));
        numbers.push(n);
    }
    let invalid_number = find_invalid_number(&numbers).unwrap();
    println!("The invalid number is {}", invalid_number);
}
