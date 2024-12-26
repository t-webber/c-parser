pub mod api {
    #![allow(clippy::pub_use)]

    pub use super::lex_content::lex_file;
    pub use super::numbers::api::Number;
    pub use super::types::lexing_data::display_tokens;
    pub use super::types::tokens_types::{Symbol, Token, TokenValue};
}
mod end_state;
mod escape;
mod lex_content;
mod numbers;
mod types;
