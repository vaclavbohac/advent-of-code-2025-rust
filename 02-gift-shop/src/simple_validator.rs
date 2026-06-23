pub fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    if !s.len().is_multiple_of(2) {
        return false;
    }

    let (first, second) = s.split_at(s.len() / 2);
    first == second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_true_for_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
    }

    #[test]
    fn it_returns_false_for_valid_id() {
        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(2332));
    }
}
