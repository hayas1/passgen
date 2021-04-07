use once_cell::sync::Lazy;
use std::collections::HashSet;

/// lower case alphabet
pub const LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
pub const LOWER_SET: Lazy<HashSet<char>> = Lazy::new(|| LOWER.chars().collect());

/// upper case alphabet
pub const UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const UPPER_SET: Lazy<HashSet<char>> = Lazy::new(|| UPPER.chars().collect());

/// numeric
pub const NUMERIC: &'static str = "0123456789";
pub const NUMERIC_SET: Lazy<HashSet<char>> = Lazy::new(|| NUMERIC.chars().collect());

/// mark
pub const DEFAULT_MARK: &'static str = "^!@#&";
pub const DEFAULT_MARK_SET: Lazy<HashSet<char>> = Lazy::new(|| DEFAULT_MARK.chars().collect());

/// mark candidate
pub const CANDIDATE_MARK: &'static str = ".,_-+=/\\^!@#&\"'$%:;><()[]{}";
pub const CANDIDATE_MARK_VEC: Lazy<Vec<char>> = Lazy::new(|| CANDIDATE_MARK.chars().collect());

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn lower_test() {
        assert_eq!(LOWER, &('a'..='z').collect::<String>());
    }

    #[test]
    fn upper_test() {
        assert_eq!(UPPER, &('A'..='Z').collect::<String>());
    }

    #[test]
    fn numeric_test() {
        assert_eq!(
            NUMERIC,
            &(0..=9).map(|i| std::char::from_digit(i, 10).unwrap()).collect::<String>(),
        )
    }

    #[test]
    fn all_mark_is_different_test() {
        assert_eq!(
            DEFAULT_MARK.chars().collect::<Vec<_>>().len(),
            DEFAULT_MARK.chars().collect::<HashSet<_>>().len()
        );
        assert_eq!(CANDIDATE_MARK.chars().collect::<HashSet<_>>().len(), CANDIDATE_MARK_VEC.len(),)
    }

    #[test]
    fn candidate_mark_contain_all_default_mark_test() {
        let mut default = HashSet::new();
        for c in CANDIDATE_MARK.chars() {
            if DEFAULT_MARK_SET.contains(&c) {
                default.insert(c);
            }
        }
        assert_eq!(default, DEFAULT_MARK_SET.clone());
    }
}
