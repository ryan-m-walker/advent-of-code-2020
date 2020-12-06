use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let passwords: Vec<_> = input.split("\n").collect();

    let old_policy_output = get_old_policy_valid_passwords_count(&passwords);
    println!("Old Policy Output: {}", old_policy_output);

    let new_policy_output = get_new_policy_valid_passwords_count(&passwords);
    println!("New Policy Output: {}", new_policy_output);
}

fn get_old_policy_valid_passwords_count(passwords: &Vec<&str>) -> u32 {
    let mut count = 0;

    for password_description in passwords {
        let (min, max, letter, password) = get_components(password_description);
        let result: Vec<_> = password.matches(&letter).collect();

        let is_greater_than = result.len() >= min;
        let is_less_than = result.len() <= max;

        if is_less_than && is_greater_than {
            count += 1;
        }
    }

    count
}

fn get_new_policy_valid_passwords_count(passwords: &Vec<&str>) -> u32 {
    let mut count = 0;

    for password_description in passwords {
        let (pos1, pos2, letter, password) = get_components(password_description);

        let char1 = password.chars().nth(pos1 - 1).unwrap().to_string();
        let char2 = password.chars().nth(pos2 - 1).unwrap().to_string();
        if xor(char1 == letter, char2 == letter) {
            count += 1;
        }
    }

    count
}

fn get_components(input: &str) -> (usize, usize, String, &str) {
    let components: Vec<_> = input.split(' ').collect();
    let min_max: Vec<usize> = components[0]
        .split('-')
        .map(|x| x.parse().unwrap())
        .collect();
    let letter = components[1].replace(":", "");
    let password = components[2];
    return (min_max[0], min_max[1], letter, password);
}

fn xor(a: bool, b: bool) -> bool {
    (a && !b) || (!a && b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        assert_eq!(get_old_policy_valid_passwords_count(&test_input), 2);
    }

    #[test]
    fn test_part_2() {
        let test_input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        assert_eq!(get_new_policy_valid_passwords_count(&test_input), 1);
    }
}
