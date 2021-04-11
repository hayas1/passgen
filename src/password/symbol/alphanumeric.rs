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

#[cfg(test)]
mod tests {
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
}
