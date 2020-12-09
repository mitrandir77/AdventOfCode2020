#![feature(str_split_once)]

use std::io::{self, BufRead};
use std::collections::BTreeSet;

fn main() {
    let stdin = io::stdin();
    let mut acc = 0;
    let mut program = vec![];
    for line in stdin.lock().lines() {
        let line = line
            .unwrap()
            .replace("+", "");
        let (op, arg) = line.split_once(' ').unwrap();

        let arg = arg.parse::<i32>().unwrap();
        program.push((op.to_string(), arg));

    }

    let mut visited = BTreeSet::new();
    let mut ip: i32 = 0;
    while ! visited.contains(&ip) {
        visited.insert(ip);
        let (op, arg) = program.get(ip as usize).unwrap();
        match &op[..] {
            "jmp" => ip += arg,
            "acc" => { acc += arg; ip+= 1 },
            "nop" => ip += 1,
            _ => panic!("unknown opcode!"),
        }
    }
    println!("acc = {}", acc);
}