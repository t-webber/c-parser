//! Implements the control flows

pub mod case;
pub mod colon_ast;
pub mod conditional;
pub mod do_while;
pub mod goto;
pub mod ident_block;
pub mod parens_block;
pub mod return_ctrl;
pub mod semi_colon;
pub mod typedef;

use core::fmt;

use crate::EMPTY;

/// Represents a block that is after a colon.
///
/// The colon was found iff `opt` is `Some(_)`.
fn repr_colon_option<T: fmt::Display>(opt: Option<&T>) -> String {
    opt.map_or_else(|| format!(" {EMPTY}"), |after| format!(": {after}"))
}
