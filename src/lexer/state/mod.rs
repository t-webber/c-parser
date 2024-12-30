pub mod api {
    #![allow(clippy::pub_use)]

    pub use super::end_state::end_current;
    pub use super::escape::{handle_escape, EscapeState};
    pub use super::lex_state::{CommentState, LexingState};
    pub use super::symbol::SymbolState;
}

mod end_state;
mod escape;
mod lex_state;
mod symbol;
