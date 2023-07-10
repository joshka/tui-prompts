#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, unused)]

mod prompt;

mod password_prompt;
mod password_state;

mod text_prompt;
mod text_state;

pub use prompt::*;

pub use text_prompt::*;
pub use text_state::*;

pub use password_prompt::*;
pub use password_state::*;

pub mod prelude {
    pub use crate::Focus;
    pub use crate::Prompt;
    pub use crate::State;
    pub use crate::Status;
}
