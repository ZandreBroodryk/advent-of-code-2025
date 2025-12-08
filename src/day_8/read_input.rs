use std::{fs, io::Error};

use crate::shared::{Coordinate3D, InputTypes};

pub fn read_input(input: Option<InputTypes>) -> Result<Vec<Coordinate3D>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_8/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let coordinates = string_contents
        .lines()
        .map(|line| {
            let coords = line
                .split(',')
                .map(|number| number.parse().expect("Incorrect input"))
                .collect::<Vec<i32>>();

            return Coordinate3D {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            };
        })
        .collect();

    Ok(coordinates)
}
