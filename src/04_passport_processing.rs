#![feature(str_split_once)]
#[macro_use]
extern crate maplit;
use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();

    let required_fields = hashset! {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
    let optional_fields = hashset! {"cid"};
    let allowed_fields: HashSet<_> = required_fields
        .union(&optional_fields)
        .copied()
        .collect();

    let mut valid_count = 0;

    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut buffer)?;
    for passport_data in buffer.split("\n\n") {
        let mut present_fields = HashSet::new();
        for field_data in passport_data.split(char::is_whitespace) {
            if let Some((field_name, _field_value)) = field_data.split_once(':') {
                present_fields.insert(field_name);
            }
        }
        if required_fields.is_subset(&present_fields) && present_fields.is_subset(&allowed_fields) {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
    Ok(())
}
