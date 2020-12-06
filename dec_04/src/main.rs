use std::collections::HashMap;
use std::fs;

extern crate regex;

use regex::Regex;

const BYR: &str = "byr";
const IYR: &str = "iyr";
const EYR: &str = "eyr";
const HGT: &str = "hgt";
const HCL: &str = "hcl";
const ECL: &str = "ecl";
const PID: &str = "pid";
const CID: &str = "cid";

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let passports: Vec<_> = input.split("\n\n").collect();
    let mut valid_keys_count = 0;
    let mut valid_passports_count = 0;
    for passport in passports {
        let parsed_passport = parse_passport(passport);
        let has_valid_keys = validate_passport_keys(&parsed_passport);
        if has_valid_keys {
            valid_keys_count += 1;
        }
        let has_valid_values = validate_passport_values(&parsed_passport);
        if has_valid_keys && has_valid_values {
            valid_passports_count += 1;
        }
    }

    println!("Output 1: {}", valid_keys_count);
    println!("Output 2: {}", valid_passports_count);
}

fn parse_passport(passport_str: &str) -> HashMap<&str, &str> {
    let mut parsed_passport = HashMap::new();
    let fields: Vec<&str> = passport_str.split_whitespace().collect();

    for field in fields {
        let parts: Vec<&str> = field.split(':').collect();
        let key = parts[0];
        let value = parts[1];

        if is_valid_key(key) {
            parsed_passport.insert(key, value);
        }
    }

    parsed_passport
}

fn validate_passport_keys(passport: &HashMap<&str, &str>) -> bool {
    let keys: Vec<_> = passport.keys().collect();
    let key_len = keys.len();

    if key_len == 8 {
        return true;
    }

    if key_len == 7 {
        if keys.iter().any(|&&x| x == "cid") {
            return false;
        }

        return true;
    }

    false
}

fn validate_passport_values(passport: &HashMap<&str, &str>) -> bool {
    for (key, field) in passport.into_iter() {
        match *key {
            BYR => {
                if !is_valid_number_range(field, 1920, 2002) {
                    return false;
                }
            }
            IYR => {
                if !is_valid_number_range(field, 2010, 2020) {
                    return false;
                }
            }
            EYR => {
                if !is_valid_number_range(field, 2010, 2030) {
                    return false;
                }
            }
            HGT => {
                if !is_valid_height(field) {
                    return false;
                }
            }
            HCL => {
                if !is_valid_hair_color(field) {
                    return false;
                }
            }
            ECL => {
                if !is_valid_eye_color(field) {
                    return false;
                }
            }
            PID => {
                if !is_valid_pid(field) {
                    return false;
                }
            }
            CID => continue,
            _ => {
                return false;
            }
        }
    }

    true
}

fn is_valid_number_range(input: &str, min: i32, max: i32) -> bool {
    let parsed_field = input.parse::<i32>().expect("Unable to parse number");
    if parsed_field < min || parsed_field > max {
        return false;
    }
    true
}

fn is_valid_height(input: &str) -> bool {
    if input.ends_with("cm") {
        let height = input
            .replace("cm", "")
            .parse::<i32>()
            .expect("Unable to parse number");
        if height >= 150 && height <= 193 {
            return true;
        }
    }

    if input.ends_with("in") {
        let height = input
            .replace("in", "")
            .parse::<i32>()
            .expect("Unable to parse number");
        if height >= 59 && height <= 76 {
            return true;
        }
    }

    false
}

fn is_valid_hair_color(input: &str) -> bool {
    let re = Regex::new(r"^#[a-fA-F0-9]{6}").unwrap();
    re.is_match(input)
}

fn is_valid_eye_color(input: &str) -> bool {
    match input {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn is_valid_pid(input: &str) -> bool {
    if input.len() != 9 {
        return false;
    }

    match input.parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_valid_key(key: &str) -> bool {
    match key {
        BYR | IYR | EYR | HGT | HCL | ECL | PID | CID => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_number_range_test() {
        assert_eq!(is_valid_number_range("2002", 1920, 2002), true);
        assert_eq!(is_valid_number_range("2003", 1920, 2002), false);
        assert_eq!(is_valid_number_range("1920", 1920, 2002), true);
        assert_eq!(is_valid_number_range("1919", 1920, 2002), false);
    }

    #[test]
    fn is_valid_height_test() {
        assert_eq!(is_valid_height("60in"), true);
        assert_eq!(is_valid_height("190cm"), true);
        assert_eq!(is_valid_height("190in"), false);
        assert_eq!(is_valid_height("190"), false);
        assert_eq!(is_valid_height("150cm"), true);
        assert_eq!(is_valid_height("149cm"), false);
        assert_eq!(is_valid_height("59in"), true);
        assert_eq!(is_valid_height("58cm"), false);
        assert_eq!(is_valid_height("193cm"), true);
        assert_eq!(is_valid_height("194cm"), false);
        assert_eq!(is_valid_height("76in"), true);
        assert_eq!(is_valid_height("77in"), false);
    }

    #[test]
    fn is_valid_hair_color_test() {
        assert_eq!(is_valid_hair_color("#123abc"), true);
        assert_eq!(is_valid_hair_color("#123abz"), false);
        assert_eq!(is_valid_hair_color("123abc"), false);
    }

    #[test]
    fn is_valid_pid_test() {
        assert_eq!(is_valid_pid("000000001"), true);
        assert_eq!(is_valid_pid("0123456789"), false);
    }
}
