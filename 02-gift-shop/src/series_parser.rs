use std::ops::RangeInclusive;

pub fn parse_series(input: &str) -> RangeInclusive<u64> {
    let split: Vec<&str> = input.split("-").collect();

    let start = split[0].parse::<u64>().unwrap();
    let end = split[1].parse::<u64>().unwrap();

    start..=end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_series() {
        let range = parse_series("11-22");

        assert_eq!(11..=22, range);
    }
}
