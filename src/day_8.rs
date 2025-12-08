use std::collections::HashMap;

use itertools::Itertools;

use crate::day_8::read_input::read_input;
use crate::shared::InputTypes;

pub mod read_input;

pub fn part_1(input: InputTypes) -> usize {
    let connection_count = match input {
        InputTypes::Example => 10,
        InputTypes::MyInput => 1000,
    };
    let input = read_input(Some(input)).expect("Error reading Input");
    let number_of_coordinates = input.len();

    let mut distances = vec![];

    for index_1 in 0..(number_of_coordinates - 1) {
        let coordinate = &input[index_1];

        for index_2 in index_1 + 1..number_of_coordinates {
            let reference = &input[index_2];

            let distance = coordinate.distance_to_point(reference);

            distances.push((distance, (coordinate, reference)));
        }
    }

    distances.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mut connected_circuits = HashMap::new();

    for index in 0..input.len() {
        let junction = input[index];
        connected_circuits.insert(junction, index);
    }

    for index in 0..connection_count {
        let distance = distances[index];

        let (_, junctions) = distance;
        let connection_1 = &connected_circuits.get(junctions.0).unwrap();
        let connection_2 = &connected_circuits.get(junctions.1).unwrap();

        if connection_1 == connection_2 {
            continue;
        }

        let mut new_connected_circuits = HashMap::new();

        for (junction, connection) in &connected_circuits {
            let value = if &connection == connection_2 {
                *connection_1
            } else {
                connection
            };
            new_connected_circuits.insert(*junction, *value);
        }
        connected_circuits = new_connected_circuits;
    }

    let mut circuits = connected_circuits
        .into_values()
        .into_group_map_by(|value| *value)
        .iter()
        .map(|grouping| grouping.1.len())
        .collect::<Vec<usize>>();
    circuits.sort();
    circuits.reverse();

    let _ = circuits.split_off(3);

    return circuits.iter().product();
}
pub fn part_2(input: InputTypes) -> usize {
    let input = read_input(Some(input)).expect("Error reading Input");
    let number_of_coordinates = input.len();

    let mut distances = vec![];

    for index_1 in 0..(number_of_coordinates - 1) {
        let coordinate = &input[index_1];

        for index_2 in index_1 + 1..number_of_coordinates {
            let reference = &input[index_2];

            let distance = coordinate.distance_to_point(reference);

            distances.push((distance, (coordinate, reference)));
        }
    }

    distances.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mut connected_circuits = HashMap::new();

    for index in 0..input.len() {
        let junction = input[index];
        connected_circuits.insert(junction, index);
    }

    for distance in distances {
        let (_, junctions) = distance;
        let connection_1 = &connected_circuits.get(junctions.0).unwrap();
        let connection_2 = &connected_circuits.get(junctions.1).unwrap();

        if connection_1 == connection_2 {
            continue;
        }

        let mut new_connected_circuits = HashMap::new();

        for (junction, connection) in &connected_circuits {
            let value = if &connection == connection_2 {
                *connection_1
            } else {
                connection
            };
            new_connected_circuits.insert(*junction, *value);
        }

        let mut is_single_circuit = true;
        for (_, circuit) in &new_connected_circuits {
            if circuit != *connection_1 {
                is_single_circuit = false;
                break;
            }
        }
        if is_single_circuit {
            return junctions.0.x as usize * junctions.1.x as usize;
        }
        connected_circuits = new_connected_circuits;
    }

    return 0;
}
