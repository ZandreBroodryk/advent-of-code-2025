use std::cmp;
use std::collections::HashSet;

use crate::day_5::read_input::read_input;
use crate::shared::InputTypes;

pub mod read_input;

pub struct IdRange {
    pub start: u64,
    pub end: u64,
}

impl IdRange {
    fn has_overlap(&self, range_reference: &IdRange) -> bool {
        if self.start >= range_reference.start && self.start <= range_reference.end {
            return true;
        }
        if self.end >= range_reference.start && self.end <= range_reference.end {
            return true;
        }
        if range_reference.start >= self.start && range_reference.start <= self.end {
            return true;
        }
        if range_reference.end >= self.start && range_reference.end <= self.end {
            return true;
        }

        return false;
    }

    fn get_range_size(&self) -> u64 {
        self.end - self.start + 1
    }
}

pub fn part_1(input: InputTypes) -> usize {
    let (fresh_ids, products) = read_input(Some(input)).expect("error reading input");

    let mut fresh_products = 0;

    for product_id in products {
        for id_range in &fresh_ids {
            if product_id >= id_range.start && product_id <= id_range.end {
                fresh_products += 1;
                break;
            }
        }
    }

    return fresh_products;
}

pub fn part_2(input: InputTypes) -> u64 {
    let (mut fresh_ids, _) = read_input(Some(input)).expect("error reading input");

    loop {
        let mut squashed_ranges = vec![];
        let mut excluded_indexes = HashSet::new();
        let mut has_overlapping_ranges = false;

        for index in 0..fresh_ids.len() {
            if excluded_indexes.contains(&index) {
                continue;
            }
            let range = &fresh_ids[index];
            let mut new_range = IdRange {
                start: range.start,
                end: range.end,
            };

            for index in index + 1..fresh_ids.len() {
                if excluded_indexes.contains(&index) {
                    continue;
                }
                let range_reference = &fresh_ids[index];
                if new_range.has_overlap(range_reference) {
                    has_overlapping_ranges = true;
                    new_range.start = cmp::min(new_range.start, range_reference.start);
                    new_range.end = cmp::max(new_range.end, range_reference.end);
                    excluded_indexes.insert(index);
                }
            }
            squashed_ranges.push(new_range);
        }

        if !has_overlapping_ranges {
            break;
        }

        fresh_ids = squashed_ranges;
    }

    return fresh_ids.iter().map(|range| range.get_range_size()).sum();
}
