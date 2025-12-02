use std::{fs, io::Error};

use crate::shared::InputTypes;

pub fn read_input(input: Option<InputTypes>) -> Result<Vec<i16>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_1/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let result = string_contents.lines().map(|line| {
        let (direction, count) = line.split_at(1);
        let count = count.parse::<i16>().expect("Incorrect input format");
        let direction = if direction == "R" { 1 } else { -1 };

        return count * direction;
    })
    .collect();
    Ok(result)
}
