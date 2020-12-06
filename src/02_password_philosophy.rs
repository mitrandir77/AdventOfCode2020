#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Line;
use std::io::{self, BufRead};

fn main() {
    let mut valid_passwords = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let_scan!(line.unwrap(); (let first_position: usize, "-", let second_position: usize, " ", let policy_letter: char, ": ", let password: Line<String>));

        let mut count = 0;
        for (i, letter) in password.chars().enumerate() {
            if letter == policy_letter && (i + 1 == first_position || i + 1 == second_position) {
                count += 1;
            }
        }
        if count == 1 {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}
