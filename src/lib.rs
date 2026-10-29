mod error;
mod input;
mod style;
mod validator;

pub use error::InputError;
pub use input::InputField;
pub use style::InputStyle;
pub use validator::CharValidator;

pub mod prelude {
    pub use super::{CharValidator, InputField, InputStyle};
}
