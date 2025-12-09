use std::collections::HashMap;

use crate::day_9::read_input::read_input;
use crate::shared::{Coordinate, InputTypes};

pub mod read_input;

pub fn part_1(input: InputTypes) -> i64 {
    let input = read_input(Some(input)).expect("Error reading Input");

    let mut largest_area = 0;

    for index in 0..input.len() - 1 {
        let corner_1 = input[index];
        for index in index + 1..input.len() {
            let corner_2 = input[index];

            let delta_x = (corner_1.x - corner_2.x).abs() + 1;
            let delta_y = (corner_1.y - corner_2.y).abs() + 1;

            let area = delta_x as i64 * delta_y as i64;

            if area > largest_area {
                largest_area = area;
            }
        }
    }
    return largest_area;
}

pub fn part_2(input: InputTypes) -> i64 {
    let input = read_input(Some(input)).expect("Error reading Input");

    let mut y_ranges_at_given_x = HashMap::new();
    let mut x_ranges_at_given_y = HashMap::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for coordinate in &input {
        let x_coordinates = y_ranges_at_given_x.entry(coordinate.x).or_insert(vec![]);
        x_coordinates.push(coordinate.y);
        let y_coordinates = x_ranges_at_given_y.entry(coordinate.y).or_insert(vec![]);
        y_coordinates.push(coordinate.x);

        if coordinate.x > max_x {
            max_x = coordinate.x;
        }
        if coordinate.y > max_y {
            max_y = coordinate.x;
        }
    }

    let mut y_ranges_at_given_x = y_ranges_at_given_x
        .iter_mut()
        .map(|entry| {
            entry.1.sort();
            (*entry.0, (entry.1[0], entry.1[1]))
        })
        .collect::<Vec<(i32, (i32, i32))>>();

    y_ranges_at_given_x.sort_by(|a, b| a.0.cmp(&b.0));

    let mut x_ranges_at_given_y = x_ranges_at_given_y
        .iter_mut()
        .map(|entry| {
            entry.1.sort();
            (*entry.0, (entry.1[0], entry.1[1]))
        })
        .collect::<Vec<(i32, (i32, i32))>>();

    x_ranges_at_given_y.sort_by(|a, b| a.0.cmp(&b.0));

    let mut largest_area = 0;

    for index_1 in 0..(input.len() - 1) {
        let corner_1 = input[index_1];
        for index_2 in (index_1 + 1)..input.len() {
            let corner_2 = input[index_2];
            if is_rectangle_inside_area(
                &corner_1,
                &corner_2,
                &y_ranges_at_given_x,
                &x_ranges_at_given_y,
            ) {
                let delta_x = (corner_1.x - corner_2.x).abs() + 1;
                let delta_y = (corner_1.y - corner_2.y).abs() + 1;
                let area = delta_x as i64 * delta_y as i64;

                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }

    return largest_area;
}

fn is_rectangle_inside_area(
    corner_1: &Coordinate,
    corner_2: &Coordinate,
    vertical_lines_at_given_x: &Vec<(i32, (i32, i32))>,
    horizontal_lines_at_given_y: &Vec<(i32, (i32, i32))>,
) -> bool {
    let rectangle_max_x = corner_1.x.max(corner_2.x);
    let rectangle_min_x = corner_1.x.min(corner_2.x);
    let rectangle_max_y = corner_1.y.max(corner_2.y);
    let rectangle_min_y = corner_1.y.min(corner_2.y);
    let above_below_bound = vertical_lines_at_given_x.iter().all(
        |(bounding_line_x, (bounding_line_y_min, bounding_line_y_max))| {
            let rectangle_left_of_bound = rectangle_max_x <= *bounding_line_x;
            let rectangle_right_of_bound = rectangle_min_x >= *bounding_line_x;
            let rectangle_above_bound = rectangle_max_y <= *bounding_line_y_min;
            let rectangle_below_bound = rectangle_min_y >= *bounding_line_y_max;
            rectangle_left_of_bound
                || rectangle_right_of_bound
                || rectangle_above_bound
                || rectangle_below_bound
        },
    );
    let left_right_bound = horizontal_lines_at_given_y.iter().all(
        |(bounding_line_y, (bounding_line_x_min, bounding_line_x_max))| {
            let rectangle_left_of_bound = rectangle_max_x <= *bounding_line_x_min;
            let rectangle_right_of_bound = rectangle_min_x >= *bounding_line_x_max;
            let rectangle_above_bound = rectangle_max_y <= *bounding_line_y;
            let rectangle_below_bound = rectangle_min_y >= *bounding_line_y;
            rectangle_left_of_bound
                || rectangle_right_of_bound
                || rectangle_above_bound
                || rectangle_below_bound
        },
    );

    return above_below_bound && left_right_bound;
}
