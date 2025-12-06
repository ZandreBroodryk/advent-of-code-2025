use std::{fs, io::Error};

use crate::{
    day_6::{MathProblem, Operation},
    shared::InputTypes,
};

pub fn read_input(input: Option<InputTypes>) -> Result<Vec<MathProblem>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_6/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let mut lines = string_contents.lines().collect::<Vec<&str>>();

    let signs = lines
        .pop()
        .expect("Incorrect input format, no text")
        .replace(" ", "");

    let mut result = signs
        .chars()
        .map(|operator| {
            let operator = match operator {
                '+' => Operation::Plus,
                '*' => Operation::Multiply,
                _ => panic!("Incorrect input format"),
            };
            return MathProblem {
                numbers: vec![],
                operator,
            };
        })
        .collect::<Vec<MathProblem>>();

    for line in lines {
        let mut math_problem_index = 0;

        for number in line.split(" ") {
            if number.len() == 0 {
                continue;
            }

            let number = number.parse().expect("Incorrect input format");
            result[math_problem_index].numbers.push(number);
            math_problem_index += 1;
        }
    }

    Ok(result)
}

pub fn read_input_cephalopod(input: Option<InputTypes>) -> Result<Vec<MathProblem>, Error> {
    let input = input.unwrap_or(InputTypes::Example);
    let path = format!("src/day_6/{}", input.to_file_name());
    let string_contents = fs::read_to_string(path)?;

    let mut lines = string_contents.lines().collect::<Vec<&str>>();

    let signs = lines.pop().expect("Incorrect input format, no text");

    let mut result = signs
        .replace(" ", "")
        .chars()
        .rev()
        .map(|operator| {
            let operator = match operator {
                '+' => Operation::Plus,
                '*' => Operation::Multiply,
                _ => panic!("Incorrect input format"),
            };
            return MathProblem {
                numbers: vec![],
                operator,
            };
        })
        .collect::<Vec<MathProblem>>();

    let mut math_problem_index = 0;
    for x in 0..signs.len() {
        let mut number = String::new();
        for y in 0..lines.len() {
            let line = lines[y].chars().rev().collect::<Vec<char>>();
            let char_to_append = line[x];

            if char_to_append != ' ' {
                number += &char_to_append.to_string();
            }
        }

        if number.len() == 0 {
            continue;
        }

        result[math_problem_index]
            .numbers
            .push(number.parse().expect("Input Format incorrect"));

        if signs.chars().rev().collect::<Vec<char>>()[x] != ' ' {
            math_problem_index += 1
        }
    }

    Ok(result)
}
