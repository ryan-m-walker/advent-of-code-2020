use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let boarding_passes: Vec<_> = input.split('\n').collect();

    let mut highest = 0;
    let mut seat_ids = vec![];

    for boarding_pass in boarding_passes {
        let seat_id = parse_boarding_pass(boarding_pass);
        if seat_id > highest {
            highest = seat_id
        }
        seat_ids.push(seat_id);
    }

    println!("highest: {}", highest);

    seat_ids.sort();

    for seat_number in 0..seat_ids.len() {
        let current_seat = seat_ids.get(seat_number);
        let next_seat = seat_ids.get(seat_number + 1);

        if current_seat.is_some() && next_seat.is_some() {
            let seat_id = current_seat.unwrap() + 1;
            let next_seat_id = next_seat.unwrap();
            if next_seat_id != &(seat_id + 1) {
                match seat_ids.get(seat_number + 2) {
                    Some(x) => {
                        if x == &(seat_id + 2) {
                            println!("Seat id: {}", seat_id + 1);
                        }
                    }
                    None => continue,
                }
            }
        }
    }
}

fn parse_boarding_pass(boarding_pass: &str) -> u32 {
    let chars: Vec<_> = boarding_pass.chars().collect();
    let row = binary_search(chars[..7].to_vec(), 127, 'F');
    let column = binary_search(chars[7..10].to_vec(), 7, 'L');

    row * 8 + column
}

fn binary_search(chars: Vec<char>, total: u32, lower: char) -> u32 {
    let mut start = 0;
    let mut end = total;

    for current in chars {
        let new_index = (end - start) / 2 + start;
        if current == lower {
            end = new_index;
        } else {
            start = new_index + 1;
        }
    }

    start
}
