#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Line;
use std::io::{self, BufRead};

fn main() {
    let mut valid_passwords = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let_scan!(line.unwrap(); (let lower_bound: i32, "-", let upper_bound: i32, " ", let policy_letter: char, ": ", let password: Line<String>));

        let mut count = 0;
        for letter in password.chars() {
            if letter == policy_letter {
                count += 1;
            }
        }
        if count <= upper_bound && count >= lower_bound {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}
