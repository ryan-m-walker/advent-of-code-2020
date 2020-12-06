use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("something went wrong reading the file");
    let numbers: Vec<u32> = input.split("\n").map(|n| n.parse().unwrap()).collect();

    match sum_two(&numbers) {
        Some(x) => println!("Answer for #1 is {}", x),
        None => println!("No answer found for #1"),
    }

    match sum_three(&numbers) {
        Some(x) => println!("Answer for #2 is {}", x),
        None => println!("No answer found for #2"),
    }
}

fn sum_two(input: &Vec<u32>) -> Option<u32> {
    for a in input {
        for b in input {
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }

    None
}

fn sum_three(input: &Vec<u32>) -> Option<u32> {
    for a in input {
        for b in input {
            for c in input {
                if a + b + c == 2020 {
                    return Some(a * b * c);
                }
            }
        }
    }

    None
}
