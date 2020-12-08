use anyhow::Result;
use std::io::{self, Read};
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut buffer)?;
    let mut sum_of_union_counts = 0;
    let mut sum_of_product_counts = 0;
    for group_answers in buffer.split("\n\n") {
        let mut group_answers_union = HashSet::new();
        let mut maybe_group_answers_product: Option<HashSet<char>> = None;
        for person_answers in group_answers.split("\n") {
            let mut person_answer_set = HashSet::new();
            for answer in person_answers.chars() {
                group_answers_union.insert(answer);
                person_answer_set.insert(answer);
            }
            if let Some(group_answers_product) =  maybe_group_answers_product {
                maybe_group_answers_product = Some(group_answers_product.intersection(&person_answer_set).copied().collect());
            }
            else {
                maybe_group_answers_product = Some(person_answer_set);
            }
        }
        sum_of_union_counts += group_answers_union.len();
        sum_of_product_counts += maybe_group_answers_product.unwrap_or(HashSet::new()).len();
    }
    println!("Anyone answered yes: {}", sum_of_union_counts);
    println!("Everyone answered yes: {}", sum_of_product_counts);
    Ok(())
}