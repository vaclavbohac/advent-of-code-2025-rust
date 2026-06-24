use std::collections::{HashMap, HashSet};

const START: char = 'S';
const SPLITTER: char = '^';

pub fn count_splits(input: &str) -> u64 {
    let mut beams: HashSet<usize> = HashSet::new();

    let mut splits: u64 = 0;

    for line in input.lines() {
        for (i, ch) in line.chars().enumerate() {
            if ch == START {
                beams.insert(i);
                continue;
            }

            if ch == SPLITTER && beams.contains(&i) {
                splits += 1;
                beams.remove(&i);
                beams.insert(i - 1);
                beams.insert(i + 1);
            }
        }
    }

    splits
}

pub fn count_quantum_timelines(input: &str) -> u64 {
    let mut counts: HashMap<usize, u64> = HashMap::new();

    for line in input.lines() {
        for (i, ch) in line.chars().enumerate() {
            if ch == START {
                counts.insert(i, 1);
                continue;
            }

            if ch == SPLITTER
                && let Some(count) = counts.remove(&i)
            {
                *counts.entry(i - 1).or_insert(0) += count;
                *counts.entry(i + 1).or_insert(0) += count;
            }
        }
    }

    counts.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::tachyon_manifold::{count_quantum_timelines, count_splits};

    #[test]
    fn test_count_splits() {
        let input = include_str!("../resources/sample-input.txt");

        assert_eq!(count_splits(input), 21);
    }

    #[test]
    fn test_count_quantum_timelines() {
        let input = include_str!("../resources/sample-input.txt");

        assert_eq!(count_quantum_timelines(input), 40);
    }
}
