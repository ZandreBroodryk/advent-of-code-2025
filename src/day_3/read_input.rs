use std::{fs, io::Error};

use crate::shared::InputTypes;

pub fn read_input(input: Option<InputTypes>) -> Result<Vec<Vec<u32>>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_3/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let result = string_contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| {
                    let number = character.to_digit(10).expect("Invalid Character");
                    return number;
                })
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    Ok(result)
}
