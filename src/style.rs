use ratatui::style::Color;

#[derive(Clone, Debug)]
pub struct InputStyle {
    pub focused: Color,
    pub unfocused: Color,
    pub error: Color,
    pub processing: Color,
    pub text_color: Color,
    pub placeholder_color: Color,
}

impl Default for InputStyle {
    fn default() -> Self {
        Self {
            focused: Color::Cyan,
            unfocused: Color::Gray,
            error: Color::Red,
            processing: Color::Yellow,
            text_color: Color::White,
            placeholder_color: Color::DarkGray,
        }
    }
}

impl InputStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_focused(mut self, color: Color) -> Self {
        self.focused = color;
        self
    }

    pub fn with_error(mut self, color: Color) -> Self {
        self.error = color;
        self
    }

    pub fn with_processing(mut self, color: Color) -> Self {
        self.processing = color;
        self
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn with_placeholder_color(mut self, color: Color) -> Self {
        self.placeholder_color = color;
        self
    }
}
