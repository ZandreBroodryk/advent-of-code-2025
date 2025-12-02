use std::{fs, io::Error};

use crate::shared::InputTypes;

pub fn read_input(input: Option<InputTypes>) -> Result<Vec<u64>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_2/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let ranges = string_contents
        .split(",")
        .map(|line| {
            let (min, max) = line.split_at(
                line.find("-")
                    .expect("Incorrect input format, must be 2 numbers separated by a -"),
            );
            let min = min.parse::<u64>().expect("Invalid input characters");
            let max = max.replace("-", "").parse::<u64>().expect("Invalid input characters");

            return (min, max);
        })
        .collect::<Vec<(u64, u64)>>();

    let mut result = vec![];
    for (min, max) in ranges {
        for value in min..=max {
            result.push(value);
        }
    }
    Ok(result)
}
