pub mod generator;
pub mod password;
pub mod symbol;

use thiserror;

/// min length of password is 8 for convenience
pub const PASSWORD_MIN_LENGTH: usize = 8;

/// default length of password is 20
pub const PASSWORD_DEFAULT_LENGTH: usize = 20;

/// max length of password is 128 for convenience such as GUI
pub const PASSWORD_MAX_LENGTH: usize = 128;

/// the size of array must be known at compile time, so we have set it to 1024
pub const PASSWORD_BUFFER_SIZE: usize = 1024;

/// password is zero-filled at dropped time
pub const PASSWORD_FILL_CHARACTER: char = '0';

#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("password should not be empty")]
    EmptyLength,

    #[error("password should be longer than {}, but given is {0}", PASSWORD_MIN_LENGTH)]
    TooShortLength(usize),

    #[error(
        "password max length is {}, for convenience such as GUI, but given is {0}",
        PASSWORD_MAX_LENGTH
    )]
    TooLongLength(usize),

    #[error("because no available symbol, cannot generate a password")]
    EmptySymbol,
}
#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("max password length is {}, but required length is {0}", PASSWORD_BUFFER_SIZE)]
    TooLongLength(usize),
}
