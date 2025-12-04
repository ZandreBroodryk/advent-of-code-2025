use crate::day_4::read_input::read_input;
use crate::shared::{Coordinate, InputTypes};

pub mod read_input;

pub fn part_1(input: InputTypes) -> u32 {
    let floor = read_input(Some(input)).expect("Error reading input data");

    let mut movable_boxes = 0;
    for tile in &floor {
        if tile.contents == '@' {
            let coordinates = get_adjacent_coordinates(&tile.coordinate);
            let mut box_count = 0;
            for coordinate in coordinates {
                let tile = floor.iter().find(|tile| tile.coordinate == coordinate);
                if let Some(tile) = tile {
                    if tile.contents == '@' {
                        box_count += 1;
                    }
                }
            }

            if box_count < 4 {
                movable_boxes += 1
            }
        }
    }
    return movable_boxes;
}

fn get_adjacent_coordinates(coordinate: &Coordinate) -> Vec<Coordinate> {
    let mut coordinates = vec![];

    for x in coordinate.x - 1..=coordinate.x + 1 {
        for y in coordinate.y - 1..=coordinate.y + 1 {
            if x == coordinate.x && y == coordinate.y {
                continue;
            }
            coordinates.push(Coordinate { x, y });
        }
    }

    return coordinates;
}
