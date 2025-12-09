use std::{fs, io::Error};

use crate::shared::{Coordinate, InputTypes};

pub fn read_input(input: Option<InputTypes>) -> Result<Vec<Coordinate>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_9/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let mut result = vec![];

    for line in string_contents.lines() {
        let numbers = line.split(",").collect::<Vec<&str>>();
        let x = numbers[0].parse().expect("Incorrect input format");
        let y = numbers[1].parse().expect("Incorrect input format");

        result.push(Coordinate { x, y });
    }

    Ok(result)
}
