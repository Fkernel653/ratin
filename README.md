# ratin — Custom input field for Ratatui TUI

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/ratin?style=for-the-badge&logo=rust&logoColor=white&label=Crates.io&color=fc8d62)](https://crates.io/crates/ratin)
[![Docs.rs](https://img.shields.io/docsrs/ratin?style=for-the-badge&logo=docs.rs&logoColor=white&label=Docs.rs&color=1e6f9f)](https://docs.rs/ratin)
[![License](https://img.shields.io/badge/License-LGPLv3-00b96b?style=for-the-badge&logo=gnu&logoColor=white)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-9cf?style=for-the-badge&logo=windows&logoColor=white)](<>)
[![Rustfmt](https://img.shields.io/badge/Code%20Style-Rustfmt-261230?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/rust-lang/rustfmt)

**ratin** is a modular input field component for [Ratatui](https://github.com/ratatui-org/ratatui) terminal applications with character validation, cursor management, and customizable styling.

---

## Table of Contents

- [Quick Start](#quick-start)
- [Features](#features)
  - [Input Field](#input-field)
  - [Character Validator](#character-validator)
  - [Styling](#styling)
- [Usage Examples](#usage-examples)
  - [Basic Input](#basic-input)
  - [URL Input](#url-input)
  - [Numeric Input](#numeric-input)
  - [Custom Character Set](#custom-character-set)
  - [Custom Styling](#custom-styling)
  - [Event Handling](#event-handling)
- [Module Structure](#module-structure)
- [Key Features](#key-features)
- [License](#license)

---

## Quick Start

```bash
cargo add ratin
```

```rust
use ratin::{InputField, CharValidator};

let validator = CharValidator::new()
    .allow_url_chars()
    .with_max_length(200);

let mut input = InputField::new()
    .with_validator(validator)
    .with_placeholder("Enter URL...")
    .with_max_length(100);
```

---

## Features

### Input Field

| Feature              | Description                        |
| -------------------- | ---------------------------------- |
| `new()`              | Create new input field             |
| `with_placeholder()` | Set placeholder text               |
| `with_block()`       | Customize block widget             |
| `with_max_length()`  | Set maximum input length           |
| `with_validator()`   | Set character validator            |
| `get_text()`         | Get current input text             |
| `clear()`            | Clear input field                  |
| `set_processing()`   | Set processing state (locks input) |
| `set_focused()`      | Set focus state                    |
| `set_error()`        | Set error message                  |

### Character Validator

| Method                  | Description                     |
| ----------------------- | ------------------------------- |
| `new()`                 | Create new validator            |
| `with_allowed_chars()`  | Set allowed characters (vector) |
| `allow_letters()`       | Allow only letters A-Z, a-z     |
| `allow_digits()`        | Allow only digits 0-9           |
| `allow_alphanumeric()`  | Allow letters and digits        |
| `allow_url_chars()`     | Allow URL-safe characters       |
| `allow_email_chars()`   | Allow email-safe characters     |
| `with_case_sensitive()` | Toggle case sensitivity         |

### Styling

| Method                     | Description                 |
| -------------------------- | --------------------------- |
| `new()`                    | Create new style            |
| `with_focused()`           | Set focused border color    |
| `with_error()`             | Set error border color      |
| `with_processing()`        | Set processing border color |
| `with_text_color()`        | Set text color              |
| `with_placeholder_color()` | Set placeholder color       |

---

## Usage Examples

<details>
<summary>Basic Input</summary>

```rust
use ratin::InputField;

let mut input = InputField::new()
    .with_placeholder("Enter your name...")
    .with_max_length(50);
```

</details>

<details>
<summary>URL Input</summary>

```rust
use ratin::{InputField, CharValidator};

let validator = CharValidator::new()
    .allow_url_chars()
    .with_case_sensitive(false);

let mut input = InputField::new()
    .with_validator(validator)
    .with_placeholder("https://example.com...")
    .with_max_length(200);
```

</details>

<details>
<summary>Numeric Input</summary>

```rust
use ratin::{InputField, CharValidator};

let validator = CharValidator::new()
    .allow_digits()
    .with_max_length(10);

let mut input = InputField::new()
    .with_validator(validator)
    .with_placeholder("Enter number...")
    .with_max_length(10);
```

</details>

<details>
<summary>Custom Character Set</summary>

```rust
use ratin::{InputField, CharValidator};

let validator = CharValidator::new()
    .with_allowed_chars(vec!['a', 'b', 'c', '1', '2', '3']);

let mut input = InputField::new()
    .with_validator(validator)
    .with_placeholder("Enter a/b/c or 1/2/3...");
```

</details>

<details>
<summary>Custom Styling</summary>

```rust
use ratin::{InputField, InputStyle};
use ratatui::style::Color;

let style = InputStyle::new()
    .with_focused(Color::Green)
    .with_error(Color::Red)
    .with_processing(Color::Yellow)
    .with_text_color(Color::White)
    .with_placeholder_color(Color::DarkGray);

let mut input = InputField::new()
    .with_style(style)
    .with_placeholder("Styled input...");
```

</details>

<details>
<summary>Event Handling</summary>

```rust
use crossterm::event::{KeyCode};

loop {
    if let Event::Key(key) = event::read()? {
        if key.code == KeyCode::Enter {
            if let Err(e) = input.validate() {
                input.set_error(Some(e.to_string()));
            } else {
                let text = input.get_text();
                input.set_processing(true);
                // Process input...
            }
        } else if key.code == KeyCode::Esc {
            break;
        } else {
            input.handle_key(key.code, key.modifiers);
        }
    }

    terminal.draw(|f| {
        input.render(f, area);
    })?;
}
```

</details>

---

## Module Structure

```
ratin/
└── src/
    ├── lib.rs          # Public API
    ├── input.rs        # Main InputField logic
    ├── style.rs        # Styling and themes
    ├── validator.rs    # Character validation
    └── error.rs        # Error types
```

---

## Key Features

- **Character Validation** — Restrict input to specific characters
- **Cursor Management** — Full cursor control with arrow keys
- **Placeholder Support** — Show hints when empty
- **Processing State** — Lock input during async operations
- **Error Handling** — Visual error feedback
- **Max Length** — Prevent overflow
- **Case Sensitivity** — Optional case-sensitive validation
- **Clean API** — Builder pattern for configuration

---

## License

[LGPLv3 License](LICENSE) — Built with:

| Library                                                | Purpose         |
| ------------------------------------------------------ | --------------- |
| [ratatui](https://github.com/ratatui-org/ratatui)      | TUI framework   |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Terminal events |

**Author:** [Fkernel653](https://github.com/Fkernel653)

**Project:** [GitHub](https://github.com/Fkernel653/ratin) • [Crates.io](https://crates.io/crates/ratin) • [Docs.rs](https://docs.rs/ratin)
