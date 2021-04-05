/// lower case alphabet
pub const LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";

/// upper case alphabet
pub const UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// numeric
pub const NUMERIC: &'static str = "0123456789";

/// mark
pub const DEFAULT_MARK: &'static str = "^!@#&";

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
    }
}
