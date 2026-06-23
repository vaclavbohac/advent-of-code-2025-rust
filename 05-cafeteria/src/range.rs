use std::ops::RangeInclusive;

pub fn range_size(range: &RangeInclusive<u64>) -> u64 {
    if range.end() >= range.start() {
        range.end() - range.start() + 1
    } else {
        0
    }
}

pub fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| *r.start());

    let mut merged = Vec::new();
    let mut current = ranges[0].clone();

    for r in ranges.into_iter().skip(1) {
        if *r.start() <= *current.end() + 1 {
            let new_end = (*current.end()).max(*r.end());
            current = *current.start()..=new_end
        } else {
            merged.push(current);
            current = r;
        }
    }

    merged.push(current);

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_size() {
        assert_eq!(range_size(&(0u64..=1)), 2);
        assert_eq!(range_size(&(0u64..=0)), 1);
        assert_eq!(range_size(&(1u64..=0)), 0);
    }

    #[test]
    fn test_merge_ranges() {
        let a = 0..=5;
        let b = 1..=10;

        assert_eq!(merge_ranges(vec![a, b]), vec![0..=10]);
    }
}
