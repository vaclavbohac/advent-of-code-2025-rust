pub fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    let doubled = format!("{}{}", s, s);
    // strip the first and last char so we don't match s at its own ends,
    // then see if s still appears
    doubled[1..doubled.len() - 1].contains(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_true_for_invalid_id() {
        assert!(is_invalid_id(111111));
        assert!(is_invalid_id(2121212121));
    }

    #[test]
    fn it_returns_false_for_valid_id() {
        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(2332));
    }
}
