use crate::range::{merge_ranges, range_size};
use std::ops::RangeInclusive;

#[derive(PartialEq, Debug)]
pub struct Inventory {
    fresh_ingredient_ranges: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

impl Inventory {
    pub fn count_fresh_ingredients(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|id| self.fresh_ingredient_ranges.iter().any(|r| r.contains(id)))
            .count()
    }

    pub fn count_unique_ids_in_fresh_ingredient_ranges(&self) -> u64 {
        merge_ranges(self.fresh_ingredient_ranges.clone())
            .iter()
            .map(range_size)
            .sum()
    }
}

fn parse_range(s: &str) -> RangeInclusive<u64> {
    let (start, end) = s.split_once('-').unwrap();

    start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
}

pub fn parse_inventory(input: &str) -> Inventory {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut numbers: Vec<u64> = Vec::new();

    let mut parsing_ranges = true;

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            ranges.push(parse_range(trimmed));
        } else {
            numbers.push(trimmed.parse::<u64>().unwrap());
        }
    }

    Inventory {
        fresh_ingredient_ranges: ranges,
        ingredients: numbers,
    }
}

#[cfg(test)]
mod tests {
    use crate::inventory::{Inventory, parse_inventory};
    use std::ops::RangeInclusive;

    #[test]
    fn test_parse_inventory() {
        let input = include_str!("../resources/sample-input.txt");

        let expected_ranges: Vec<RangeInclusive<u64>> = vec![3..=5, 10..=14, 16..=20, 12..=18];

        let expected_numbers: Vec<u64> = vec![1, 5, 8, 11, 17, 32];

        let inventory = Inventory {
            fresh_ingredient_ranges: expected_ranges,
            ingredients: expected_numbers,
        };

        assert_eq!(parse_inventory(input), inventory);
    }

    #[test]
    fn test_count_fresh_ingredients() {
        let input = include_str!("../resources/sample-input.txt");

        let inventory = parse_inventory(input);

        assert_eq!(inventory.count_fresh_ingredients(), 3);
    }

    #[test]
    fn test_count_fresh_ingredient_ranges() {
        let input = include_str!("../resources/sample-input.txt");

        let inventory = parse_inventory(input);

        assert_eq!(inventory.count_unique_ids_in_fresh_ingredient_ranges(), 14);
    }
}
