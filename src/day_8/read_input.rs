use std::{fs, io::Error};

use crate::shared::InputTypes;

pub fn read_input(input: Option<InputTypes>) -> Result<String, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_8/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    Ok(string_contents)
}
