use std::{collections::HashSet, fs, io::Error};

use crate::{day_5::IdRange, shared::InputTypes};

pub fn read_input(input: Option<InputTypes>) -> Result<(Vec<IdRange>, HashSet<u64>), Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_5/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let lines = string_contents.lines();

    let mut fresh_ids = vec![];
    let mut checked_ids = HashSet::new();

    for line in lines {
        if line.contains("-") {
            let (start, end) = line.split_at(line.find("-").unwrap());
            let start = start.parse::<u64>().expect("Incorrect input format");
            let end = end
                .replace("-", "")
                .parse::<u64>()
                .expect("Incorrect input format");

            fresh_ids.push(IdRange { start, end });
            continue;
        }

        if line.len() == 0 {
            continue;
        }
        let number = line
            .parse::<u64>()
            .expect(format!("incorrect input format: {line}").as_str());
        checked_ids.insert(number);
    }

    Ok((fresh_ids, checked_ids))
}
