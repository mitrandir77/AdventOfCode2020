use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut max = 0;
    let mut seat_ids = Vec::new();
    for line in stdin.lock().lines() {
        let binary_num = line
            .unwrap()
            .replace("F", "0")
            .replace("B", "1")
            .replace("R", "1")
            .replace("L", "0");
        let seat_id = isize::from_str_radix(&binary_num, 2).unwrap();
        if seat_id > max {
            max = seat_id;
        }
        seat_ids.push(seat_id);
    }
    seat_ids.sort_unstable();
    let mut my_seat_id = -1;
    for (i, seat_id) in seat_ids[0..seat_ids.len() - 1].iter().enumerate() {
        if *seat_id + 1 != *seat_ids.get(i + 1).unwrap_or(&0) {
            my_seat_id = *seat_id + 1;
        }
    }
    println!("max seat id: {}", max);
    println!("my seat id: {}", my_seat_id);
}
