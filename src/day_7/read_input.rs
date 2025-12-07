use std::{fs, io::Error};

use crate::{
    day_7::TachyonManifoldSpace,
    shared::{InputTypes, Space2D},
};

pub fn read_input(input: Option<InputTypes>) -> Result<Space2D<TachyonManifoldSpace>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_7/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;
    let lines = string_contents.lines().collect::<Vec<&str>>();

    let mut result = Space2D(vec![]);

    for y in 0..lines.len() {
        let mut row = vec![];
        let characters = lines[y].chars().collect::<Vec<char>>();
        for x in 0..lines[y].len() {
            row.push(match characters[x] {
                'S' => TachyonManifoldSpace::Start,
                '.' => TachyonManifoldSpace::EmptySpace,
                '^' => TachyonManifoldSpace::Splitter,
                _ => panic!("Invalid input format, input can only contain 'S' '.' and '^', found {}", characters[x]),
            });
        }
        result.0.push(row);
    }

    Ok(result)
}
