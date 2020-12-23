#[macro_use]
extern crate scan_rules;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().collect();
    let_scan!(lines.get(0).unwrap().as_ref().unwrap(); (let timestamp:i32));
    let mut ids = vec![];
    for bus_str in lines.get(1).unwrap().as_ref().unwrap().split(',') {
        if let Ok(bus_id) = bus_str.parse::<i32>() {
            ids.push(bus_id);
        }
    }

    let mut min = i32::MAX;
    let mut min_id = 0;
    for bus_id in ids {
        let modulo = timestamp % bus_id;
        let mut depature = (timestamp / bus_id) * bus_id;
        if modulo != 0 {
            depature += bus_id;
        }
        if depature < min {
            min_id = bus_id;
            min = depature;
        }
    }
    println!("{}", (min - timestamp) * min_id);
}
