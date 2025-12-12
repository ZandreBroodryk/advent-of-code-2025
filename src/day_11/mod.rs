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
    let mut input = read_input(Some(input), 2).expect("Error reading Input");

    remove_redundant_paths_on_crucial_nodes(&mut input);
    clean_up_dead_connections(&mut input);
    shorten_single_connections(&mut input);

    let starting_point = input.get("svr").expect("no starting point");

    let mut result = 0;

    for next_step in starting_point {
        let history = vec!["svr".to_string(), next_step.clone().to_string()];
        take_path(history, next_step.clone(), &input, &mut result);
    }

    return result;
}

fn remove_redundant_paths_on_crucial_nodes(network: &mut HashMap<String, Vec<String>>) {
    for (_, splits) in network {
        if splits.contains(&"fft".to_string()) {
            *splits = vec!["fft".to_string()];
            continue;
        }
        if splits.contains(&"dac".to_string()) {
            *splits = vec!["dac".to_string()];
        }
    }
}

fn shorten_single_connections(network: &mut HashMap<String, Vec<String>>) {
    loop {
        let mut items_removed = 0;
        for (key, splits) in network.clone() {
            if key == "svr" || key == "fft" || key == "dac" {
                continue;
            }
            if splits.len() == 1 {
                let new_value = splits.first().unwrap();
                network.remove(&key);
                items_removed += 1;

                for (_, steps) in network.iter_mut() {
                    for step in steps {
                        if *step == key {
                            *step = new_value.clone();
                        }
                    }
                }
            }
        }
        if items_removed == 0 {
            return;
        }
    }
}

fn clean_up_dead_connections(network: &mut HashMap<String, Vec<String>>) {
    loop {
        let mut items_removed = 0;
        for (key, _) in network.clone() {
            if key == "svr" || key == "fft" || key == "dac" {
                continue;
            }
            let mut has_connection = false;
            for (_, splits) in network.clone() {
                if splits.contains(&key) {
                    has_connection = true;
                }
            }

            if !has_connection {
                network.remove(&key);
                items_removed += 1;
            }
        }
        if items_removed == 0 {
            return;
        }
    }
}

fn take_path(
    history: Vec<String>,
    next_step: String,
    network: &HashMap<String, Vec<String>>,
    result: &mut usize,
) {
    let next_steps = get_next_split(&next_step, network);

    if next_steps.is_empty() {
        if history.contains(&"fft".to_string()) && history.contains(&"dac".to_string()) {
            *result += 1;

            dbg!(result);
        }
        return;
    }

    for next_step in next_steps {
        if history.contains(&next_step) {
            return;
        }
        let mut history = history.clone();
        history.push(next_step.clone());
        take_path(history, next_step, network, result);
    }
}
