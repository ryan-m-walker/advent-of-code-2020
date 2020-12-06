use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let groups: Vec<_> = input.split("\n\n").collect();
    let unique_count = get_all_unique_answers_count(&groups);
    println!("Unique Count: {}", unique_count);
    let same_count = get_all_same_answers_count(&groups);
    println!("Same Count: {}", same_count)
}

fn get_all_unique_answers_count(groups: &Vec<&str>) -> u32 {
    let mut total_count = 0;

    for group in groups {
        total_count += get_all_group_count(group);
    }

    total_count
}

fn get_all_group_count(group: &str) -> u32 {
    let mut all_answers: HashMap<char, bool> = HashMap::new();
    let people: Vec<_> = group.split("\n").collect();
    for person in people {
        let answers = person.chars();
        for answer in answers {
            if !all_answers.contains_key(&answer) {
                all_answers.insert(answer, true);
            }
        }
    }

    all_answers.len() as u32
}

fn get_all_same_answers_count(groups: &Vec<&str>) -> u32 {
    let mut total_count = 0;

    for group in groups {
        total_count += get_same_group_count(group);
    }

    total_count
}

fn get_same_group_count(group: &str) -> u32 {
    let mut answers_count = 0;
    let mut all_answers: HashMap<char, u32> = HashMap::new();
    let people: Vec<_> = group.split("\n").collect();

    for person in &people {
        let answers = person.chars();
        for answer in answers {
            if !all_answers.contains_key(&answer) {
                all_answers.insert(answer, 1);
            } else {
                let old_value = all_answers.get_mut(&answer).unwrap();
                *old_value += 1;
            }
        }
    }

    for (_, count) in all_answers {
        if count == people.len() as u32 {
            answers_count += 1;
        }
    }

    answers_count
}
