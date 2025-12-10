use std::sync::{Arc, Mutex};
use std::{thread, usize};

use crate::day_10::read_input::read_input;
use crate::shared::InputTypes;

pub mod read_input;

#[derive(Debug, Clone)]
pub struct Machine {
    pub lights: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
    pub joltage: Vec<usize>,
    pub state: Vec<bool>,
    pub last_button_press: Option<usize>,
}

impl Machine {
    fn press_button_lights(&mut self, button_number: usize) {
        for light in &self.buttons[button_number] {
            self.state[*light] = !self.state[*light]
        }
        self.last_button_press = Some(button_number);
    }

    fn is_lights_in_correct_state(&self) -> bool {
        self.state == self.lights
    }

    fn get_next_light_states(&self) -> Vec<Self> {
        let mut machines = vec![];
        for button_number in 0..self.buttons.len() {
            if button_number == self.last_button_press.unwrap_or(self.buttons.len()) {
                continue;
            }

            let mut new_machine = self.clone();

            new_machine.press_button_lights(button_number);
            machines.push(new_machine);
        }

        return machines;
    }

    fn find_minimum_press_count_for_lights(&self) -> usize {
        let mut press_count = 1;
        let mut machines = self.get_next_light_states();

        loop {
            for machine in &machines {
                if machine.is_lights_in_correct_state() {
                    return press_count;
                }
            }
            press_count += 1;

            if press_count >= self.buttons.len() {
                panic!("no success after")
            }
            let mut new_states = vec![];
            for machine in &machines {
                new_states.append(&mut machine.get_next_light_states());
            }
            machines = new_states;
        }
    }
    fn press_button_joltage(&mut self, button_number: usize) -> Result<(), String> {
        for joltage_counter in &self.buttons[button_number] {
            if self.joltage[*joltage_counter] == 0 {
                return Err("Invalid button press".to_string());
            }
            self.joltage[*joltage_counter] -= 1;
        }
        return Ok(());
    }

    fn is_joltage_in_correct_state(&self) -> bool {
        self.joltage.iter().all(|counter| *counter == 0)
    }

    fn get_next_joltage_states(&self) -> Vec<Self> {
        let mut machines = vec![];
        for button_number in 0..self.buttons.len() {
            let mut new_machine = self.clone();

            if let Ok(_) = new_machine.press_button_joltage(button_number) {
                machines.push(new_machine);
            }
        }

        return machines;
    }

    fn find_minimum_press_count_for_joltage(&self) -> usize {
        let mut press_count = 1;
        let mut machines = self.get_next_joltage_states();

        loop {
            for machine in &machines {
                if machine.is_joltage_in_correct_state() {
                    return press_count;
                }
            }
            press_count += 1;

            let mut new_states = vec![];
            for machine in &machines {
                new_states.append(&mut machine.get_next_joltage_states());
            }
            machines = new_states;
        }
    }
}

pub fn part_1(input: InputTypes) -> usize {
    let input = read_input(Some(input)).expect("Error reading Input");

    let result = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for machine in input {
        let result = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let presses = machine.find_minimum_press_count_for_lights();
            let mut answer = result.lock().unwrap();

            *answer += presses;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *result.lock().unwrap();

    return result;
}

pub fn part_2(input: InputTypes) -> usize {
    let input = read_input(Some(input)).expect("Error reading Input");

    let result = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for machine in input {
        let result = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let presses = machine.find_minimum_press_count_for_joltage();
            let mut answer = result.lock().unwrap();

            *answer += presses;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *result.lock().unwrap();

    return result;
}
