#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Word;
use std::collections::HashMap;
use std::io::{self, BufRead};

enum Ins {
    Mask((u64, u64)),
    Ass {
        // Assignment
        addr: u64,
        val: u64,
    },
}

const THIRTY_SIX_BITS: u64 = 68719476735;

fn parse_mask(mask: &str) -> Ins {
    Ins::Mask((
        u64::from_str_radix(mask.replace("X", "0").as_str(), 2).unwrap(),
        u64::from_str_radix(mask.replace("X", "1").as_str(), 2).unwrap(),
    ))
}

fn apply_mask(val: u64, mask: (u64, u64)) -> u64 {
    let (ones, zeroes) = mask;
    (val | ones) & zeroes & THIRTY_SIX_BITS
}

fn main() {
    let stdin = io::stdin();
    let mut program = vec![];
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let ins = scan! {line.as_str();
            ("mask = ", let mask: Word) => parse_mask(mask),
            ("mem[", let addr: u64, "] = ", let val: u64) => Ins::Ass { addr, val},
        }
        .unwrap();
        program.push(ins);
    }

    let mut mem = HashMap::new();
    let mut mask = (0, 0);
    for ins in program {
        match ins {
            Ins::Mask(new_mask) => mask = new_mask,
            Ins::Ass { addr, val } => {
                let _ = mem.insert(addr, apply_mask(val, mask));
            }
        }
    }
    let mut sum = 0;
    for val in mem.values() {
        sum += val;
    }
    println!("{}", sum);
}
