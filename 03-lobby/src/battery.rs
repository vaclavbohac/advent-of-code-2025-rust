pub fn parse_battery(s: &str) -> Vec<u32> {
    let digits: Vec<u32> = s
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    digits
}

pub fn get_joltage(battery: Vec<u32>, cells: u32) -> u64 {
    assert!(cells > 0);
    assert!(battery.len() >= cells as usize);

    let mut result: Vec<u64> = Vec::new();

    let mut cloned = battery.clone();

    for j in (0..cells).rev() {
        let mut max_i = 0;
        let mut max = 0;
        let end = cloned.len() - j as usize;

        for i in 0..end {
            if cloned[i] > max {
                max = cloned[i];
                max_i = i;
            }
        }

        let n = 10u64.pow(j);

        result.push(max as u64 * n);

        cloned = cloned[max_i + 1..cloned.len()].to_vec();
    }

    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_battery() {
        assert_eq!(Vec::from([1, 2, 3]), parse_battery("123"));
    }

    #[test]
    fn test_get_joltage_for_single_cell() {
        let battery = Vec::from([1, 2, 3]);
        assert_eq!(3, get_joltage(battery, 1));
    }

    #[test]
    fn test_get_joltage_for_two_cells() {
        assert_eq!(23, get_joltage(vec![1, 2, 3], 2));
        assert_eq!(32, get_joltage(vec![3, 2, 1], 2));
    }

    #[test]
    fn test_get_joltage_for_three_cells() {
        assert_eq!(123, get_joltage(vec![1, 2, 3], 3));
        // assert_eq!(321, get_joltage(vec![3, 2, 1], 3));
    }

    #[test]
    fn test_get_joltage_for_12_cells() {
        let battery = Vec::from([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(987654321111, get_joltage(battery, 12));
    }
}