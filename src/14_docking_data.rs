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

fn parse_mask(mask: &str) -> Ins {
    Ins::Mask((
        u64::from_str_radix(mask.replace("X", "0").as_str(), 2).unwrap(),
        u64::from_str_radix(mask.replace("X", "1").as_str(), 2).unwrap(),
    ))
}

#[cfg(not(feature = "part_two"))]
fn apply_mask(val: u64, mask: (u64, u64)) -> u64 {
    const THIRTY_SIX_BITS: u64 = 68719476735;

    let (ones, zeroes) = mask;
    (val | ones) & zeroes & THIRTY_SIX_BITS
}

#[cfg(feature = "part_two")]
fn apply_addr_mask(addr: u64, mask: (u64, u64)) -> HashSet<u64> {
    use std::collections::HashSet;
    let (ones, zeroes) = mask;
    let ones_applied = addr | ones;

    let xs = ones ^ zeroes;

    let mut res = HashSet::new();
    res.insert(ones_applied);
    let mut bit = 1;
    for _ in 0..36 {
        if bit & xs > 0 {
            let mut to_add = vec![];
            for entry in res.iter() {
                to_add.push(entry | bit);
                to_add.push(entry & (!bit));
            }
            res.extend(to_add.into_iter());
        }

        bit <<= 1;
    }
    res
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
    #[cfg(not(feature = "part_two"))]
    {
        for ins in program {
            match ins {
                Ins::Mask(new_mask) => mask = new_mask,
                Ins::Ass { addr, val } => {
                    let _ = mem.insert(addr, apply_mask(val, mask));
                }
            }
        }
    }
    #[cfg(feature = "part_two")]
    {
        for ins in program {
            match ins {
                Ins::Mask(new_mask) => mask = new_mask,
                Ins::Ass { addr, val } => {
                    for addr in apply_addr_mask(addr, mask) {
                        mem.insert(addr, val);
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for val in mem.values() {
        sum += val;
    }
    println!("{}", sum);
}
