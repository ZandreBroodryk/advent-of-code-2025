use std::{fs, io::Error};

use crate::{day_10::Machine, shared::InputTypes};

enum Stage {
    Lights,
    Buttons,
    Joltage,
}

pub fn read_input(input: Option<InputTypes>) -> Result<Vec<Machine>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_10/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let mut result = vec![];

    for line in string_contents.lines() {
        let mut lights = vec![];
        let mut buttons = vec![];
        let mut joltage = vec![];
        let mut current_string = String::new();
        let mut current_stage = Stage::Lights;
        let mut button_number = 0;
        for character in line.chars() {
            match character {
                '[' => current_stage = Stage::Lights,
                '(' => {
                    current_stage = Stage::Buttons;
                    button_number += 1;
                }
                '{' => current_stage = Stage::Joltage,
                ',' | ']' | ')' | '}' => {
                    match current_stage {
                        Stage::Lights => {
                            lights = current_string
                                .chars()
                                .map(|character| character == '#')
                                .collect()
                        }
                        Stage::Buttons => {
                            let number = current_string.parse::<usize>().unwrap();
                            if buttons.len() < button_number {
                                buttons.push(vec![number]);
                            } else {
                                buttons[button_number - 1].push(number);
                            }
                        }
                        Stage::Joltage => joltage.push(current_string.parse::<usize>().unwrap()),
                    }
                    current_string.clear();
                }
                '.' | '#' | '0'..='9' => current_string.push(character),
                _ => {}
            }
        }
        let state = vec![false; lights.len()];
        let machine = Machine {
            buttons,
            joltage,
            lights,
            state,
            last_button_press: None
        };

        result.push(machine);
    }

    Ok(result)
}
