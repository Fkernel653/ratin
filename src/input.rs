use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::error::InputError;
use crate::style::InputStyle;
use crate::validator::CharValidator;

pub struct InputField {
    text: String,
    cursor_position: u16,
    is_focused: bool,
    is_processing: bool,
    placeholder: String,
    error_message: Option<String>,
    style: InputStyle,
    block: Option<Block<'static>>,
    validator: CharValidator,
    max_length: usize,
}

impl InputField {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor_position: 0,
            is_focused: true,
            is_processing: false,
            placeholder: "Enter text...".to_string(),
            error_message: None,
            style: InputStyle::default(),
            block: None,
            validator: CharValidator::default(),
            max_length: 0,
        }
    }

    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn with_block(mut self, block: Block<'static>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = max;
        self
    }

    pub fn with_validator(mut self, validator: CharValidator) -> Self {
        self.validator = validator;
        self
    }

    pub fn set_processing(&mut self, processing: bool) {
        self.is_processing = processing;
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error_message = error;
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_position = 0;
        self.error_message = None;
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn validate(&self) -> Result<(), InputError> {
        if self.text.is_empty() {
            return Err(InputError::Empty);
        }

        if self.max_length > 0 && self.text.len() > self.max_length {
            return Err(InputError::TooLong(self.max_length));
        }

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let border_color = if self.error_message.is_some() {
            self.style.error
        } else if self.is_processing {
            self.style.processing
        } else if self.is_focused {
            self.style.focused
        } else {
            self.style.unfocused
        };

        let mut block = self
            .block
            .clone()
            .unwrap_or_else(|| Block::default().title(" Input ").borders(Borders::ALL));

        block = block.style(Style::default().fg(border_color));

        let display_text = if self.is_processing {
            "⏳ Processing...".to_string()
        } else if self.text.is_empty() {
            self.placeholder.clone()
        } else {
            let cursor_pos = self.cursor_position as usize;
            let (before, after) = self.text.split_at(cursor_pos);
            format!("{}█{}", before, after)
        };

        let text_color = if self.is_processing {
            Color::DarkGray
        } else if self.text.is_empty() {
            self.style.placeholder_color
        } else {
            self.style.text_color
        };

        let paragraph = Paragraph::new(display_text)
            .block(block)
            .style(Style::default().fg(text_color));

        frame.render_widget(paragraph, area);

        if self.is_focused && !self.is_processing {
            let cursor_x = area.x + 1 + self.cursor_position;
            let cursor_y = area.y + 1;
            frame.set_cursor_position(Position::new(cursor_x, cursor_y));
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        if self.is_processing {
            return false;
        }

        match key {
            KeyCode::Char(c) => self.handle_char_input(c),
            KeyCode::Backspace => self.handle_backspace(),
            KeyCode::Delete => self.handle_delete(),
            KeyCode::Left => self.handle_left(),
            KeyCode::Right => self.handle_right(),
            KeyCode::Home => {
                self.cursor_position = 0;
                true
            }
            KeyCode::End => {
                self.cursor_position = self.text.len() as u16;
                true
            }
            _ => false,
        }
    }

    fn handle_char_input(&mut self, c: char) -> bool {
        if self.max_length > 0 && self.text.len() >= self.max_length {
            self.error_message = Some(format!("Maximum length: {}", self.max_length));
            return true;
        }

        if let Err(e) = self.validator.validate(c) {
            self.error_message = Some(e.to_string());
            return true;
        }

        let pos = self.cursor_position as usize;
        self.text.insert(pos, c);
        self.cursor_position += 1;
        self.error_message = None;
        true
    }

    fn handle_backspace(&mut self) -> bool {
        if self.cursor_position > 0 {
            let pos = (self.cursor_position - 1) as usize;
            self.text.remove(pos);
            self.cursor_position -= 1;
            self.error_message = None;
            true
        } else {
            false
        }
    }

    fn handle_delete(&mut self) -> bool {
        let pos = self.cursor_position as usize;
        if pos < self.text.len() {
            self.text.remove(pos);
            self.error_message = None;
            true
        } else {
            false
        }
    }

    fn handle_left(&mut self) -> bool {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            true
        } else {
            false
        }
    }

    fn handle_right(&mut self) -> bool {
        if (self.cursor_position as usize) < self.text.len() {
            self.cursor_position += 1;
            true
        } else {
            false
        }
    }
}

impl Default for InputField {
    fn default() -> Self {
        Self::new()
    }
}
