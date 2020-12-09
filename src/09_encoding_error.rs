#[macro_use]
extern crate scan_rules;

use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::ops::Range;

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
fn find_range_that_sums_up_to_n(numbers: &[i64], n: i64) -> Option<Range<usize>> {
    let mut sum = 0;
    let (mut beginning, mut end) = (0, 0);

    loop {
        if sum < n && end <= numbers.len() {
            sum += numbers[end];
            end += 1;
        } else if sum > n && beginning <= numbers.len() && beginning < end {
            sum -= numbers[beginning];
            beginning += 1;
        } else if sum == n {
            return Some(beginning..end);
        } else {
            break;
        }
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
    let weakness_range = &numbers[find_range_that_sums_up_to_n(&numbers, invalid_number).unwrap()];
    let weakness = weakness_range.iter().min().unwrap() + weakness_range.iter().max().unwrap();
    println!("The weakness is {}", weakness);
}
