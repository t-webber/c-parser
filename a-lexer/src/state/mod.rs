//! Module that contains the states used to lex.
//!
//! They contain temporary data and context information needed for the lexing.

pub use end_state::end_current;
pub use escape::{EscapeState, handle_escape};
pub use lex_state::{CommentState, LexingState};
pub use symbol::SymbolState;

mod end_state;
mod escape;
mod lex_state;
mod symbol;
