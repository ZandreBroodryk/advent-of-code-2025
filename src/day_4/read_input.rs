use std::{collections::HashMap, fs, io::Error};

use crate::shared::{Coordinate, InputTypes};

pub fn read_input(input: Option<InputTypes>) -> Result<HashMap<Coordinate, char>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_4/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let lines = string_contents.lines().collect::<Vec<&str>>();

    let mut result = HashMap::new();

    for y in 0..lines.len() {
        let chars = lines[y].chars().collect::<Vec<char>>();
        let y = y as i32;
        for x in 0..chars.len() {
            let x_coordinate = x as i32;
            let value = chars[x];
            let key = Coordinate { x: x_coordinate, y };

            result.entry(key).or_insert(value);
        }
    }

    Ok(result)
}
