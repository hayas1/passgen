use std::collections::HashSet;

use super::{password::Password, symbol, GeneratorError};

/// default length of password is 20
pub const PASSWORD_DEFAULT_LENGTH: usize = 20;

#[derive(Debug)]
pub struct PasswordGenerator {
    pub len: usize,
    pub use_lower: bool,
    pub use_upper: bool,
    pub use_numeric: bool,
    pub addition: HashSet<char>,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            len: PASSWORD_DEFAULT_LENGTH,
            use_lower: true,
            use_upper: true,
            use_numeric: true,
            addition: symbol::DEFAULT_MARK_SET.clone(),
        }
    }
}

impl PasswordGenerator {
    /// get new generator
    pub fn new(
        len: usize,
        use_lower: bool,
        use_upper: bool,
        use_numeric: bool,
        addition: HashSet<char>,
    ) -> Self {
        Self { len, use_lower, use_upper, use_numeric, addition }
    }

    /// generate password. (this method redraw until use_* is satisfied)
    pub fn generate_password(&self) -> anyhow::Result<Password> {
        self.can_generate()?;
        let password = loop {
            let pw = Password::generate(self.len, &self.get_chars())?;
            if self.validate(&pw) {
                break pw;
            }
        };
        Ok(password)
    }

    #[inline]
    /// check generator state, which can generate password
    pub fn can_generate(&self) -> anyhow::Result<()> {
        if self.len == 0 {
            Err(GeneratorError::EmptyLength)?
        } else if self.len < 8 {
            Err(GeneratorError::TooShortLength)?
        } else if self.get_chars().is_empty() {
            Err(GeneratorError::EmptySymbol)?
        } else {
            Ok(())
        }
    }

    #[inline]
    /// get password available chars, based on generator state
    pub fn get_chars(&self) -> Vec<char> {
        let mut chars = "".to_string();
        if self.use_lower {
            chars += symbol::LOWER;
        }
        if self.use_upper {
            chars += symbol::UPPER;
        }
        if self.use_numeric {
            chars += symbol::NUMERIC;
        }
        if !self.addition.is_empty() {
            chars += &self.addition.iter().collect::<String>()
        }
        chars.chars().collect()
    }

    /// validate password, such as is numeric used when use_numeric is true
    pub fn validate(&self, password: &Password) -> bool {
        let (mut used_lower, mut used_upper, mut used_numeric, mut used_addition) =
            (false, false, false, false);
        for c in password.iter() {
            used_lower |= symbol::LOWER_SET.contains(c);
            used_upper |= symbol::UPPER_SET.contains(c);
            used_numeric |= symbol::NUMERIC_SET.contains(c);
            used_addition |= self.addition.contains(c);
        }
        !(self.use_lower && !used_lower)            // if use_lower=false, addition can include lower alphabet.
            && !(self.use_upper && !used_upper)     // so, return true only if "use_lower=true and used_lower=false"
            && !(self.use_numeric && !used_numeric) // the code `!(self.use_lower && !used_lower)` can work so.
            && !self.addition.is_empty() == used_addition
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_test() {
        // password example: 7UPCItcE^#NMDKaXQHo4
        let password = PasswordGenerator::default().generate_password().unwrap();
        let (mut used_lower, mut used_upper, mut used_numeric, mut used_addition) =
            (false, false, false, false);
        for c in password.iter() {
            used_lower |= symbol::LOWER_SET.contains(c);
            used_upper |= symbol::UPPER_SET.contains(c);
            used_numeric |= symbol::NUMERIC_SET.contains(c);
            used_addition |= symbol::DEFAULT_MARK_SET.contains(c);
            assert!(
                symbol::LOWER_SET.contains(c)
                    || symbol::UPPER_SET.contains(c)
                    || symbol::NUMERIC_SET.contains(c)
                    || symbol::DEFAULT_MARK_SET.contains(c)
            )
        }
        assert!(used_lower && used_upper && used_numeric && used_addition);
    }

    #[test]
    fn generate_uniqueness_test() {
        let generator = PasswordGenerator::default();
        let mut set = HashSet::new();
        for _ in 0..500 {
            set.insert(format!("{:?}", generator.generate_password().unwrap()));
        }
        assert_eq!(set.len(), 500);
    }

    #[test]
    fn generate_error_test() {
        let mut generator = PasswordGenerator::default();
        generator.len = 0;
        assert_eq!(
            generator.generate_password().unwrap_err().to_string(),
            "password should not be empty"
        );
        generator.len = 7;
        assert_eq!(
            generator.generate_password().unwrap_err().to_string(),
            "password should be longer than 8"
        );
        generator.len = 2048;
        assert_eq!(
            generator.generate_password().unwrap_err().to_string(),
            "max password length is 1024, but required length is 2048"
        );
        generator.len = 8;
        generator.use_lower = false;
        generator.use_upper = false;
        generator.use_numeric = false;
        generator.addition.clear();
        assert_eq!(
            generator.generate_password().unwrap_err().to_string(),
            "because no available symbol, cannot generate a password"
        );
    }

    #[test]
    fn generator_setting_no_numeric_but_added_numeric_test() {
        let mut generator = PasswordGenerator::default();
        generator.use_numeric = false;
        generator.addition = (0..=9).map(|i| std::char::from_digit(i, 10).unwrap()).collect();
        let generated_password = generator.generate_password().unwrap();
        assert!(generator.validate(&generated_password));
        println!("{:?}", generated_password);
        // password example: pEamK5KhY0Ig6bB4lWZF
        // ^ generator.use_numeric=false, but addition include numeric, so password include numeric
    }

    #[test]
    fn generator_setting_only_a_test() {
        let generator =
            PasswordGenerator::new(20, false, false, false, vec!['a'].into_iter().collect());
        let generated_password = generator.generate_password().unwrap();
        assert_eq!(format!("{:?}", generated_password), "aaaaaaaaaaaaaaaaaaaa");
    }
}
