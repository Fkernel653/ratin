use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum InputError {
    Empty,
    InvalidChar(char),
    TooLong(usize),
    Custom(String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::Empty => write!(f, "Field cannot be empty"),
            InputError::InvalidChar(c) => write!(f, "Invalid character: '{}'", c),
            InputError::TooLong(max) => write!(f, "Maximum length: {}", max),
            InputError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for InputError {}
