use crate::shared::InputTypes;

use crate::day_2::read_input::read_input;

pub mod read_input;

pub fn part_1(file_type: Option<InputTypes>) -> u64 {
    let input = read_input(file_type).expect("Error reading Input");

    let mut result = vec![];

    for number in input {
        if has_repeating_sequence(number) {
            result.push(number);
        }
    }

    return result.iter().sum();
}

fn has_repeating_sequence(number: u64) -> bool {
    let number = number.to_string();

    if number.len() % 2 == 1 {
      return false;
    }

    let len = number.len() / 2;
    let segment = &number[..len];
    let new_string = segment.repeat(2);

    if new_string == number {
        return true;
    }

    return false;
}
