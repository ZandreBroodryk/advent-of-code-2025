use crate::day_6::read_input::{read_input, read_input_cephalopod};
use crate::shared::InputTypes;

pub mod read_input;

pub enum Operation {
    Plus,
    Multiply,
}

pub struct MathProblem {
    pub numbers: Vec<u64>,
    pub operator: Operation,
}

impl MathProblem {
    pub fn find_solution(&self) -> u64 {
        match self.operator {
            Operation::Plus => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product(),
        }
    }
}

pub fn part_1(input: InputTypes) -> u64 {
    let input = read_input(Some(input)).expect("Error reading Input");

    let result = input.iter().map(|problem| problem.find_solution()).sum();
    return result;
}

pub fn part_2(input: InputTypes) -> u64 {
    let input = read_input_cephalopod(Some(input)).expect("Error reading Input");

    let result = input.iter().map(|problem| problem.find_solution()).sum();
    return result;
}