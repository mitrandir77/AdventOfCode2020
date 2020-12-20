#[macro_use]
extern crate scan_rules;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let (mut x, mut y) = (0, 0);
    let mut direction = 0;
    for line in stdin.lock().lines() {
        let_scan!(line.unwrap(); (let action: char, let value: i32));
        match action {
            'N' => y += value,
            'S' => y -= value,
            'E' => x += value,
            'W' => x -= value,
            'L' => direction = (direction - value + 360) % 360,
            'R' => direction = (direction + value + 360) % 360,
            'F' => match direction {
                0 => x += value,
                90 => y -= value,
                180 => x -= value,
                270 => y += value,
                _ => panic!(format!("direction is out of sync {}!", direction)),
            },
            _ => panic!("uncrecognized action!"),
        }
    }
    println!("{}", i32::abs(x) + i32::abs(y));
}
