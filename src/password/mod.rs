pub mod password;
pub mod symbol;

use password::PASSWORD_MAX_LENGTH;
use thiserror;
#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("max password length is {}, but required length is {0}", PASSWORD_MAX_LENGTH)]
    TooLongLength(usize),
}
