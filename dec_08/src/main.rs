use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines: Vec<_> = input.split("\n").collect();
    execute_program(&lines);
    patch_program(&lines);
}

fn patch_program(program: &Vec<&str>) {
    for (i, line) in (&program).iter().enumerate() {
        if line.contains("nop") || line.contains("jmp") {
            let mut clone = program.clone();
            let (opcode, operand) = parse_operation(line);
            let new_opcode = if opcode == "nop" { "jmp" } else { "nop" };
            let patch = &(format!("{} {}", new_opcode, operand));
            clone[i] = patch;

            if execute_program(&clone) {
                return;
            }
        }
    }
}

fn execute_program(program: &Vec<&str>) -> bool {
    let mut pointer: i32 = 0;
    let mut acc: i32 = 0;
    let mut visited_lines = HashSet::new();

    loop {
        if pointer == program.len() as i32 {
            println!("program finished executing");
            println!("acc = {}", acc);
            return true;
        }

        if visited_lines.contains(&pointer) {
            println!("infinite loop encountered");
            println!("acc = {}", acc);
            return false;
        }

        visited_lines.insert(pointer);

        let operation = program.get(pointer as usize).expect("Unable to read line");
        let (opcode, operand) = parse_operation(operation);

        match opcode {
            "acc" => {
                acc += operand;
                pointer += 1;
            }
            "jmp" => {
                pointer += operand;
            }
            "nop" => {
                pointer += 1;
            }
            _ => panic!("Unexpected opcode {}", opcode),
        }
    }
}

fn parse_operation(operation: &str) -> (&str, i32) {
    let tokens: Vec<&str> = operation.split(" ").collect();
    let opcode = tokens[0];
    let operand: i32 = tokens[1].parse().unwrap();
    (opcode, operand)
}
