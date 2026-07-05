use crate::error::InputError;

#[derive(Clone, Debug)]
pub struct CharValidator {
    allowed_chars: Vec<char>,
    case_sensitive: bool,
}

impl CharValidator {
    pub fn new() -> Self {
        Self {
            allowed_chars: Vec::new(),
            case_sensitive: true,
        }
    }

    pub fn with_allowed_chars(mut self, chars: Vec<char>) -> Self {
        self.allowed_chars = chars;
        self
    }

    pub fn with_case_sensitive(mut self, sensitive: bool) -> Self {
        self.case_sensitive = sensitive;
        self
    }

    pub fn allow_letters(mut self) -> Self {
        self.allowed_chars = ('a'..='z').chain('A'..='Z').collect();
        self
    }

    pub fn allow_digits(mut self) -> Self {
        self.allowed_chars = ('0'..='9').collect();
        self
    }

    pub fn allow_alphanumeric(mut self) -> Self {
        self.allowed_chars = ('a'..='z').chain('A'..='Z').chain('0'..='9').collect();
        self
    }

    pub fn allow_url_chars(mut self) -> Self {
        self.allowed_chars = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
            'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_', '.', '~', ':', '/',
            '?', '=', '&', '%', '#', '+', '@', '!', '$', '\'', '(', ')', '*', ',', ';',
        ];
        self
    }

    pub fn allow_email_chars(mut self) -> Self {
        self.allowed_chars = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
            'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '_', '-', '@', '+',
        ];
        self
    }

    pub fn validate(&self, c: char) -> Result<(), InputError> {
        if self.allowed_chars.is_empty() {
            return Ok(());
        }

        let is_valid = if self.case_sensitive {
            self.allowed_chars.contains(&c)
        } else {
            self.allowed_chars
                .iter()
                .any(|&allowed| allowed.to_ascii_lowercase() == c.to_ascii_lowercase())
        };

        if is_valid {
            Ok(())
        } else {
            Err(InputError::InvalidChar(c))
        }
    }

    pub fn get_allowed_chars(&self) -> &[char] {
        &self.allowed_chars
    }
}

impl Default for CharValidator {
    fn default() -> Self {
        Self::new()
    }
}
