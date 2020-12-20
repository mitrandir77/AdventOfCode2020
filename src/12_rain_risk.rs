#![feature(destructuring_assignment)]
#[macro_use]
extern crate scan_rules;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let (mut x, mut y) = (0, 0); // ship position
    #[cfg(not(feature = "part_two"))]
    {
        let mut direction = 0;
        for line in stdin.lock().lines() {
            let_scan!(line.unwrap(); (let action: char, let value: i64));
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
    }

    #[cfg(feature = "part_two")]
    {
        let (mut wx, mut wy) = (10, 1); // waypoint position
        for line in stdin.lock().lines() {
            let_scan!(line.unwrap(); (let action: char, let value: i64));
            match action {
                'N' => wy += value,
                'S' => wy -= value,
                'E' => wx += value,
                'W' => wx -= value,
                'L' => {
                    for _ in 0..(value / 90) {
                        (wx, wy) = (-wy, wx);
                    }
                }
                'R' => {
                    for _ in 0..(value / 90) {
                        (wx, wy) = (wy, -wx);
                    }
                }
                'F' => {
                    let (vx, vy) = (wx, wy);
                    x += vx * value;
                    y += vy * value
                }
                _ => panic!("uncrecognized action!"),
            }
        }
    }
    println!("{}", i64::abs(x) + i64::abs(y));
}
