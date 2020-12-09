#![feature(str_split_once)]

use std::collections::BTreeSet;
use std::io::{self, BufRead};

fn run(program: &[(String, i32)]) -> (bool, i32) {
    let mut acc = 0;
    let mut visited = BTreeSet::new();
    let mut ip: i32 = 0;
    while !visited.contains(&ip) {
        if ip == program.len() as i32 {
            return (true, acc);
        }
        visited.insert(ip);
        let (op, arg) = program.get(ip as usize).unwrap();
        match &op[..] {
            "jmp" => ip += arg,
            "acc" => {
                acc += arg;
                ip += 1
            }
            "nop" => ip += 1,
            _ => panic!("unknown opcode!"),
        }
    }
    (false, acc)
}

fn main() {
    let stdin = io::stdin();
    let mut program = vec![];
    for line in stdin.lock().lines() {
        let line = line.unwrap().replace("+", "");
        let (op, arg) = line.split_once(' ').unwrap();

        let arg = arg.parse::<i32>().unwrap();
        program.push((op.to_string(), arg));
    }

    println!("acc at the end of run = {}", run(&program).1);

    for i in 0..program.len() {
        let (op, arg) = &program[i].clone();
        if op == "jmp" {
            program[i] = ("nop".to_string(), *arg);
            let (terminated, acc) = run(&program);
            if terminated {
                println!("acc at the end of run of corrected program = {}", acc);
            }
            program[i] = ("jmp".to_string(), *arg);
        }
    }
}
