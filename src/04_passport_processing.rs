#![feature(str_split_once)]
#[macro_use]
extern crate maplit;
use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn validate_year(year: &str, lower_bound: i32, upper_bound: i32) -> bool {
    if year.len() == 4 {
        if let Ok(year) = year.parse::<i32>() {
            return (lower_bound..=upper_bound).contains(&year);
        }
    }
    false
}

fn validate_hair_colour(colour: &str) -> bool {
    let re = Regex::new(r"^#[a-z0-9]{6}$").unwrap();
    re.is_match(colour)
}

fn validate_eye_colour(colour: &str) -> bool {
    let valid_colours = hashset! { "amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
    valid_colours.contains(colour)
}

fn validate_height(height: &str) -> bool {
    let re = Regex::new(r"^(\d+)((cm)|(in))$").unwrap();
    if let Some(captures) = re.captures(height) {
        let height = captures[1].parse::<i32>().unwrap();
        if &captures[2] == "in" {
            (59..=76).contains(&height)
        } else {
            (150..=193).contains(&height)
        }
    } else {
        false
    }
}

fn validate_passport_id(id: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(id)
}

fn main() -> Result<()> {
    let mut buffer = String::new();

    let required_fields = hashset! {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
    let optional_fields = hashset! {"cid"};
    let allowed_fields: HashSet<_> = required_fields.union(&optional_fields).copied().collect();

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
        if required_fields.is_subset(&present_fields)
            && present_fields.is_subset(&allowed_fields)
            && validate_year(parsed_passport_data.get("byr").unwrap(), 1920, 2002)
            && validate_year(parsed_passport_data.get("iyr").unwrap(), 2010, 2020)
            && validate_year(parsed_passport_data.get("eyr").unwrap(), 2020, 2030)
            && validate_height(parsed_passport_data.get("hgt").unwrap())
            && validate_hair_colour(parsed_passport_data.get("hcl").unwrap())
            && validate_eye_colour(parsed_passport_data.get("ecl").unwrap())
            && validate_passport_id(parsed_passport_data.get("pid").unwrap())
        {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
    Ok(())
}
