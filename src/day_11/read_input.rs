use std::{collections::HashMap, fs, io::Error};

use crate::shared::InputTypes;

pub fn read_input(input: Option<InputTypes>, part_number: u8) -> Result<HashMap<String, Vec<String>>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_11/{}", input.to_file_name_with_part(part_number));
    let string_contents = fs::read_to_string(path)?;

    let mut result = HashMap::new();
    for line in string_contents.lines() {
        let (device, connections) = line.split_at(
            line.find(|char| char == ':')
                .expect("Incorrect input format"),
        );

        let connections = connections
            .replace(": ", "")
            .split(" ")
            .map(|entry| entry.to_string())
            .collect();

        result.insert(device.to_string(), connections);
    }

    Ok(result)
}
