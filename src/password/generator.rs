use std::collections::HashSet;

use super::{password::Password, symbol, GeneratorError};

/// default length of password is 20
pub const PASSWORD_DEFAULT_LENGTH: usize = 20;

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
    /// generate password, based on generator state
    pub fn generate(&self) -> anyhow::Result<Password> {
        self.check()?;
        let password = loop {
            let pw = Password::generate(&self.get_chars(), self.len)?;
            if self.validate(&pw) {
                break pw;
            }
        };
        Ok(password)
    }

    #[inline]
    /// check generator state, which can generate password
    pub fn check(&self) -> anyhow::Result<()> {
        if self.len == 0 {
            Err(GeneratorError::EmptyLength)?
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
        self.use_lower == used_lower
            && self.use_upper == used_upper
            && self.use_numeric == used_numeric
            && !self.addition.is_empty() == used_addition
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_test() {
        let generator = PasswordGenerator::default();
        println!("{:?}", generator.generate());
    }
}
