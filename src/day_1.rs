use crate::{day_1::read_input::read_input, shared::InputTypes};

pub mod read_input;

pub fn part_1(file_type: Option<InputTypes>) -> u16 {
    let input = read_input(file_type).expect("Error reading input");
    let mut current_value: i16 = 50;
    let mut count = 0;

    for line in input {
        current_value += line;
        if current_value < 0 {
            current_value = 100 + (current_value % 100);
        }
        if current_value > 99 {
            current_value = current_value % 100;
        }

        if current_value == 0 {
            count += 1;
        }
        // dbg!(current_value);
    }

    return count;
}

pub fn part_2(file_type: Option<InputTypes>) -> u16 {
    let input = read_input(file_type).expect("Error reading input");
    let mut current_value: i16 = 50;
    let mut count = 0;

    for line in input {
        let step = if line > 0 { 1 } else { -1 };
        for _ in 1..=line.abs() {
            current_value += step;
            if current_value == 100 {
                current_value = 0;
            }
            if current_value == -1 {
                current_value = 99;
            }
            if current_value == 0 {
                count += 1;
            }
        }
    }

    return count;
}
