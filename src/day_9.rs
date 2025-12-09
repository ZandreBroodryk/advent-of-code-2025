use crate::day_9::read_input::read_input;
use crate::shared::InputTypes;

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
