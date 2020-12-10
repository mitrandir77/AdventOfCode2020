#[macro_use]
extern crate scan_rules;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut numbers = Vec::new();
    for line in stdin.lock().lines() {
        let_scan!(line.unwrap(); (let n: i64));
        numbers.push(n);
    }
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);

    let mut ones = 0;
    let mut threes = 0;
    for (a, b) in numbers[0..numbers.len() - 1]
        .iter()
        .zip(&numbers[1..numbers.len()])
    {
        match b - a {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    println!("numbers of 1's times number of 3's: {}", ones * threes);
    let mut combinations = vec![0 as i64; (*numbers.last().unwrap() + 1) as usize];

    combinations[0] = 1;
    for n in &numbers[1..numbers.len()] {
        combinations[*n as usize] += combinations[(n - 1) as usize];
        if n - 2 >= 0 {
            combinations[*n as usize] += combinations[(n - 2) as usize];
        }
        if n - 3 >= 0 {
            combinations[*n as usize] += combinations[(n - 3) as usize];
        }
    }
    println!(
        "Number of possible adapter arrangements: {}",
        combinations.last().unwrap()
    );
}
