use std::collections::HashMap;

use crate::day_11::read_input::read_input;
use crate::shared::InputTypes;

pub mod read_input;

pub fn part_1(input: InputTypes) -> usize {
    let input = read_input(Some(input)).expect("Error reading Input");

    let starting_point = input.get("you").expect("no starting point");

    let mut result = 0;

    let mut points_to_check = starting_point.clone();
    loop {
        let mut next_points_to_check = vec![];
        for point in points_to_check {
            let new_points_to_check = get_next_split(&point, &input);
            for point in new_points_to_check {
                if point == "out" {
                    result +=1;
                } else {
                    next_points_to_check.push(point);
                }
            }
        }
        if next_points_to_check.is_empty() {
            break;
        }
        points_to_check = next_points_to_check;
    }

    return result;
}

fn get_next_split(current_position: &str, network: &HashMap<String, Vec<String>>) -> Vec<String> {
    network.get(current_position).unwrap_or(&Vec::new()).clone()
}

pub fn part_2(input: InputTypes) -> usize {
    let input = read_input(Some(input)).expect("Error reading Input");

    return input.len();
}
