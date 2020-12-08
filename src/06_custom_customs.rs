use anyhow::Result;
use std::io::{self, Read};
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut buffer)?;
    let mut sum_of_counts = 0;
    for group_answers in buffer.split("\n\n") {
        let mut group_answers_union = HashSet::new();
        for person_answers in group_answers.split("\n") {
            for answer in person_answers.chars() {
                group_answers_union.insert(answer);
            }
        }
        sum_of_counts += group_answers_union.len();
    }
    println!("{}", sum_of_counts);
    Ok(())
}