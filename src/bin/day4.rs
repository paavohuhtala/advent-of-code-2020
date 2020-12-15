use std::collections::HashMap;

use regex;

fn main() {
    let input = include_str!("input4.txt");
    println!("4b {:?}", day4b(input));
}

fn day4b(input: &str) -> usize {
    let passport_values_regex = regex::Regex::new(r"([a-z]+):([a-z0-9#]+)[\n ]?").unwrap();
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut field_validators: HashMap<&str, fn(&str) -> bool> = HashMap::new();

    field_validators.insert("byr", |year: &str| {
        let year: u32 = year.parse().unwrap();
        year >= 1920 && year <= 2002
    });

    field_validators.insert("iyr", |year: &str| {
        let year: u32 = year.parse().unwrap();
        year >= 2010 && year <= 2020
    });

    field_validators.insert("eyr", |year: &str| {
        let year: u32 = year.parse().unwrap();
        year >= 2020 && year <= 2030
    });

    field_validators.insert("hgt", |field: &str| {
        match field
            .char_indices()
            .skip_while(|(_, ch)| ch.is_ascii_digit())
            .next()
        {
            None => return false,
            Some((unit_offset, _)) => {
                let (height, unit) = field.split_at(unit_offset);
                let height: u32 = height.parse().unwrap();

                match unit {
                    "cm" => height >= 150 && height <= 193,
                    "in" => height >= 59 && height <= 76,
                    _ => panic!(),
                }
            }
        }
    });

    field_validators.insert("hcl", |color| {
        let hex_color_regex = regex::Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        hex_color_regex.is_match(color)
    });

    field_validators.insert("ecl", |color| {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&color)
    });

    field_validators.insert("pid", |pid| {
        let pid_regex = regex::Regex::new(r"^[0-9]{9}$").unwrap();
        pid_regex.is_match(pid)
    });

    let mut valid = 0;

    'main: for full_passport in input.split("\n\n") {
        let mut passport: HashMap<String, String> = HashMap::new();

        for capture in passport_values_regex.captures_iter(full_passport) {
            let key = capture.get(1).unwrap();
            let value = capture.get(2).unwrap();
            passport.insert(key.as_str().to_string(), value.as_str().to_string());
        }

        for key in &keys {
            let entry = passport.get(&key.to_string());
            match entry {
                None => {
                    continue 'main;
                }
                Some(value) => {
                    if !(field_validators.get(key).unwrap())(value) {
                        continue 'main;
                    }
                }
            }
        }

        valid += 1;
    }

    valid
}
