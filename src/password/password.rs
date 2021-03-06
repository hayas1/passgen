use super::{PasswordError, PASSWORD_BUFFER_SIZE, PASSWORD_FILL_CHARACTER};
use anyhow;
use itertools;
use rand::Rng;

pub struct Password {
    len: usize,
    password: [char; PASSWORD_BUFFER_SIZE],
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
    pub fn generate(len: usize, chars: &[char]) -> anyhow::Result<Self> {
        if len > PASSWORD_BUFFER_SIZE {
            Err(PasswordError::TooLongLength(len))?
        }
        let mut csp_rng = rand::thread_rng();
        let mut password = [PASSWORD_FILL_CHARACTER; PASSWORD_BUFFER_SIZE];
        let indices = (0..len).map(|_| csp_rng.gen_range(0, chars.len())); // rand 0.7
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

    /// if return &str, we have to do some allocations, so there is no choice but to return String
    pub fn to_string(&self) -> String {
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
        let a = Password::generate(10, &['a']).unwrap();
        assert_eq!(format!("{}", a), "********");
        assert_eq!(format!("{:?}", a), "aaaaaaaaaa");
    }

    #[test]
    fn generate_uniqueness_test() {
        let chars = format!("{}{}{}", symbol::LOWER, symbol::UPPER, symbol::NUMERIC);
        let mut set = HashSet::new();
        for _ in 0..10000 {
            set.insert(
                Password::generate(20, &chars.chars().collect::<Vec<_>>()).unwrap().to_string(),
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
        let valid = Password::generate(1024, &['a', 'b', 'c']);
        assert!(valid.is_ok());
        let invalid = Password::generate(1025, &['a', 'b', 'c']);
        assert!(invalid.is_err());
        assert_eq!(
            invalid.unwrap_err().to_string(),
            "max password length is 1024, but required length is 1025",
        )
    }
}
