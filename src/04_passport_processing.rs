#![feature(str_split_once)]
#[macro_use]
extern crate maplit;
use anyhow::Result;
use std::collections::{HashSet, HashMap};
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
        let mut parsed_passport_data = HashMap::new();
        for field_data in passport_data.split(char::is_whitespace) {
            if let Some((field_name, field_value)) = field_data.split_once(':') {
                parsed_passport_data.insert(field_name, field_value);
            }
        }
        let present_fields: HashSet<_> = parsed_passport_data.keys().copied().collect();
        if required_fields.is_subset(&present_fields) && present_fields.is_subset(&allowed_fields) {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
    Ok(())
}
