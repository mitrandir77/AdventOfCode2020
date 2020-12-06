fn main() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut x = 0; // current tobogan position
    let mut tree_count = 0;

    let mut lines = stdin.lock().lines();
    lines.next();
    for line in lines {
        let line = line.unwrap();
        x += 3;
        x %= line.len();
        if line.chars().nth(x) == Some('#') {
            tree_count += 1;
        }
    }
    println!("{}", tree_count);
}
