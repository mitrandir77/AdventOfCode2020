#[macro_use]
extern crate scan_rules;
use std::collections::HashMap;

fn main() {
    let_readln!([ let numbers: usize ],+);
    let mut spoken_before = HashMap::new();
    for (turn, number) in numbers.iter().enumerate() {
        spoken_before.insert(*number, turn);
    }
    let mut prev_number = *numbers.as_slice().last().unwrap();
    for turn in numbers.len()..30000000 {
        let current_number = match spoken_before.get(&prev_number) {
            Some(previous_turn) => turn - 1 - previous_turn,
            None => 0,
        };
        spoken_before.insert(prev_number, turn - 1);
        prev_number = current_number;
    }
    println!("{}", prev_number);
}
