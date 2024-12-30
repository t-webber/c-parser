//! Module that contains the states used to lex.
//!
//! They contain temporary data and context information needed for the lexing.

pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use)]

    pub use super::end_state::end_current;
    pub use super::escape::{EscapeState, handle_escape};
    pub use super::lex_state::{CommentState, LexingState};
    pub use super::symbol::SymbolState;
}

mod end_state;
mod escape;
mod lex_state;
mod symbol;
