use std::collections::HashMap;

use crate::day_11::read_input::read_input;
use crate::shared::InputTypes;

pub mod read_input;

pub fn part_1(input: InputTypes) -> usize {
    let input = read_input(Some(input), 1).expect("Error reading Input");

    let starting_point = input.get("you").expect("no starting point");

    let mut result = 0;

    let mut points_to_check = starting_point.clone();
    loop {
        let mut next_points_to_check = vec![];
        for point in points_to_check {
            let new_points_to_check = get_next_split(&point, &input);
            for point in new_points_to_check {
                if point == "out" {
                    result += 1;
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
    let input = read_input(Some(input), 2).expect("Error reading Input");

    let starting_point = input.get("svr").expect("no starting point");

    let mut result = 0;

    for next_step in starting_point {
        let history = vec!["svr".to_string(), next_step.clone().to_string()];
        take_path(history, next_step.clone(), &input, &mut result);
    }

    return result;
}

fn take_path(
    history: Vec<String>,
    next_step: String,
    network: &HashMap<String, Vec<String>>,
    result: &mut usize,
) {
    let next_steps = get_next_split(&next_step, network);

    if next_steps.is_empty() {
        return;
    }

    for next_step in next_steps {
        if history.contains(&next_step) {
            return;
        }

        if next_step == "out" {
            if history.contains(&"fft".to_string()) && history.contains(&"dac".to_string()) {
                *result += 1;
            }
            return;
        }
        let mut history = history.clone();
        history.push(next_step.clone());
        take_path(history, next_step, network, result);
    }
}
