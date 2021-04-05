use super::PasswordError;
use anyhow;
use itertools;
use rand::Rng;

/// the size of array must be known at compile time, so we have set it to 1024.
pub const PASSWORD_MAX_LENGTH: usize = 1024;

/// password is zero-filled at dropped time.
pub const PASSWORD_FILL_CHARACTER: char = '0';

pub struct Password {
    len: usize,
    password: [char; PASSWORD_MAX_LENGTH],
}

impl Drop for Password {
    fn drop(&mut self) {
        for i in 0..self.len() {
            self.password[i] = PASSWORD_FILL_CHARACTER;
        }
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", vec!['*'; 8].into_iter().collect::<String>())
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Password {
    /// generate new password, from given chars, with CSPRNG
    pub fn generate(chars: &[char], len: usize) -> anyhow::Result<Self> {
        if len > PASSWORD_MAX_LENGTH {
            Err(PasswordError::TooLongLength(len))?
        }
        let mut csp_rng = rand::thread_rng();
        let mut password = [PASSWORD_FILL_CHARACTER; PASSWORD_MAX_LENGTH];
        let indices = (0..len).map(|_| csp_rng.gen_range(0..chars.len()));
        for (i, j) in itertools::zip(0..len, indices) {
            password[i] = chars[j];
        }
        Ok(Self { len, password })
    }

    #[inline]
    /// return password length
    pub fn len(&self) -> usize {
        self.len
    }

    /// security reason, this method is private, but this might be public
    fn to_string(&self) -> String {
        self.password.iter().take(self.len()).collect()
    }

    /// return iterator for password
    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.password.iter().take(self.len())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::super::symbol;
    use super::*;

    #[test]
    fn generate_test() {
        let a = Password::generate(&['a'], 10).unwrap();
        assert_eq!(format!("{}", a), "********");
        assert_eq!(format!("{:?}", a), "aaaaaaaaaa");
    }

    #[test]
    fn generate_uniqueness_test() {
        let chars = format!("{}{}{}", symbol::LOWER, symbol::UPPER, symbol::NUMERIC);
        let mut set = HashSet::new();
        for _ in 0..10000 {
            set.insert(
                Password::generate(&chars.chars().collect::<Vec<_>>(), 20).unwrap().to_string(),
            );
        }
        assert_eq!(set.len(), 10000);
    }

    #[test]
    fn drop_test() {
        let raw: *const _;
        {
            let mut password = ['0'; 1024];
            password[0] = 'a';
            password[1] = 'b';
            password[2] = 'c';
            let password = Password { len: 3, password };
            raw = &password.password;
            assert_eq!(unsafe { &*raw }.clone()[..3], ['a', 'b', 'c']);
        }
        assert_eq!(unsafe { &*raw }.clone()[..3], ['0', '0', '0']);
    }

    #[test]
    fn too_long_password_test() {
        let valid = Password::generate(&['a', 'b', 'c'], 1024);
        assert!(valid.is_ok());
        let invalid = Password::generate(&['a', 'b', 'c'], 1025);
        assert!(invalid.is_err());
        assert_eq!(
            invalid.unwrap_err().to_string(),
            "max password length is 1024, but required length is 1025",
        )
    }
}
