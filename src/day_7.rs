use std::collections::HashSet;

use crate::day_7::read_input::read_input;
use crate::shared::{Coordinate, InputTypes};

pub mod read_input;

#[derive(PartialEq, Eq, Hash)]
pub enum TachyonManifoldSpace {
    EmptySpace,
    Start,
    Splitter,
    Beam,
}

pub fn part_1(input: InputTypes) -> usize {
    let input = read_input(Some(input)).expect("Error reading Input");

    let mut beams = HashSet::new();
    let mut beam_split_count = 0;

    for y in 0..input.0.len() {
        for x in 0..input.0[y].len() {
            let value = &input.0[y][x];
            let current_coordinate = Coordinate {
                x: x as i32,
                y: y as i32,
            };
            let coordinate_above = Coordinate {
                x: x as i32,
                y: y as i32 - 1,
            };
            let has_beam_above = beams.contains(&coordinate_above);
            match value {
                TachyonManifoldSpace::EmptySpace => {
                    if has_beam_above {
                        beams.insert(current_coordinate);
                    }
                }
                TachyonManifoldSpace::Start => {
                    beams.insert(current_coordinate);
                }
                TachyonManifoldSpace::Splitter => {
                    if has_beam_above {
                        beam_split_count += 1;
                        beams.insert(Coordinate {
                            x: x as i32 - 1,
                            y: y as i32,
                        });
                        beams.insert(Coordinate {
                            x: x as i32 + 1,
                            y: y as i32,
                        });
                    }
                }
                TachyonManifoldSpace::Beam => continue,
            }
        }
    }

    return beam_split_count;
}
