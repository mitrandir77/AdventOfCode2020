use std::io::{self, BufRead};

fn check_slope(lines: &[String], right: usize, down: usize) -> i64 {
    let mut x = 0; // current tobogan position
    let mut tree_count = 0;
    for (line_no, line) in lines.iter().skip(down).enumerate() {
        if line_no % down == 0 {
            x += right;
            x %= line.len();
            if line.chars().nth(x) == Some('#') {
                tree_count += 1;
            }
        }
    }
    tree_count
}

fn main() {
    let stdin = io::stdin();

    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    println!(
        "{}",
        check_slope(&lines, 1, 1)
            * check_slope(&lines, 3, 1)
            * check_slope(&lines, 5, 1)
            * check_slope(&lines, 7, 1)
            * check_slope(&lines, 1, 2)
    );
}
