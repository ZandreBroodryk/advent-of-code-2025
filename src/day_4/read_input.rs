use std::{fs, io::Error};

use crate::shared::{Coordinate, InputTypes, Tile};

pub fn read_input(input: Option<InputTypes>) -> Result<Vec<Tile>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_4/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let lines = string_contents.lines().collect::<Vec<&str>>();

    let mut result = vec![];

    for y in 0..lines.len() {
        let chars = lines[y].chars().collect::<Vec<char>>();
        let y = y as i32;
        for x in 0..chars.len() {
            let x_coordinate = x as i32;
            result.push(Tile {
                coordinate: Coordinate { x: x_coordinate, y },
                contents: chars[x],
            });
        }
    }

    Ok(result)
}
