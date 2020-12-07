use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut max = 0;
    for line in stdin.lock().lines() {
        let binary_num = line
            .unwrap()
            .replace("F", "0")
            .replace("B", "1")
            .replace("R", "1")
            .replace("L", "0");
        let intval = isize::from_str_radix(&binary_num, 2).unwrap();
        if intval > max {
            max = intval;
        }
    }
    println!("{}", max);
}
