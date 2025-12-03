use std::cmp::max;

use crate::shared::InputTypes;

use crate::day_3::read_input::read_input;

pub mod read_input;

pub fn part_1(input: InputTypes) -> u32 {
    let banks = read_input(Some(input)).expect("Error reading input data");
    let max_values = banks.iter().map(|bank| find_max_combo(bank));

    return max_values.sum();
}

fn find_max_combo(bank: &Vec<u32>) -> u32 {
    let bank_length = bank.len();

    let mut max_first_digit_value = 0;
    let mut max_first_digit_index = 0;
    let mut max_second_digit_value = 0;

    for index in 0..bank_length - 1 {
        let battery_joltage = bank[index as usize];
        if max_first_digit_value < battery_joltage {
            max_first_digit_value = battery_joltage;
            max_first_digit_index = index + 1;
        }
    }

    for index in max_first_digit_index..bank_length {
        let battery_joltage = bank[index];

        max_second_digit_value = max(battery_joltage, max_second_digit_value);
    }

    let max_voltage = (max_first_digit_value * 10) + max_second_digit_value;

    return max_voltage;
}

pub fn part_2(input: InputTypes) -> u64 {
    let banks = read_input(Some(input)).expect("Error reading input data");
    let max_values = banks
        .iter()
        .map(|bank| remove_smallest_digits(bank, 12))
        .collect::<Vec<u64>>();

    return max_values.into_iter().sum();
}

fn remove_smallest_digits(bank: &Vec<u32>, digit_count: usize) -> u64 {
    if bank.len() <= digit_count {
        return bank
            .iter()
            .map(|digit| digit.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .expect("Error creating number");
    }
    let mut result = vec![];
    let mut previous_max_index = 0;
    for digit_number in (0..digit_count).rev() {
        let mut max_value = 0;
        let mut max_index = bank.len();
        let starting_index = previous_max_index;
        for index in starting_index..bank.len() - digit_number {
            let value = bank[index];
            if max_value < value {
                max_value = value;
                max_index = index;
            }
        }
        previous_max_index = max_index + 1;
        result.push(bank[max_index]);
    }

    return result
        .iter()
        .map(|digit| digit.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .expect("Error creating number");
}
