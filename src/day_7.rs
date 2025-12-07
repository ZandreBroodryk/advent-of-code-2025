use std::collections::HashSet;

use crate::day_7::read_input::read_input;
use crate::shared::{Coordinate, InputTypes, Space2D};

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

pub fn part_2(input: InputTypes) -> usize {
    let input = read_input(Some(input)).expect("Error reading Input");

    for x in 0..input.0[0].len() {
        let coordinate = Coordinate { x: x as i32, y: 0 };
        let space = input.get_coordinate(&coordinate).unwrap();

        if *space == TachyonManifoldSpace::Start {
            let result = count_time_splits_from_position(&coordinate, &input);
            return result;
        }
    }

    return 0;
}

fn count_time_splits_from_position(
    current_beam_coordinate: &Coordinate,
    area: &Space2D<TachyonManifoldSpace>,
) -> usize {
    let next_coordinate =
        current_beam_coordinate.get_next_in_direction(&crate::shared::Direction::Down);
    let next_space = area.get_coordinate(&next_coordinate);

    if let Some(space) = next_space {
        match space {
            TachyonManifoldSpace::EmptySpace => {
                return count_time_splits_from_position(&next_coordinate, area);
            }
            TachyonManifoldSpace::Start => {
                return count_time_splits_from_position(&next_coordinate, area);
            }
            TachyonManifoldSpace::Splitter => {
                let option_1 =
                    next_coordinate.get_next_in_direction(&crate::shared::Direction::Left);

                let time_splits_for_option_1 = count_time_splits_from_position(&option_1, area);

                let option_2 =
                    next_coordinate.get_next_in_direction(&crate::shared::Direction::Right);

                let time_splits_for_option_2 = count_time_splits_from_position(&option_2, area);

                return time_splits_for_option_1 + time_splits_for_option_2;
            }
            TachyonManifoldSpace::Beam => {
                return count_time_splits_from_position(&next_coordinate, area);
            }
        }
    } else {
        return 1;
    }
}
