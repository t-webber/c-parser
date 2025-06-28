//! Defines the brace-block nodes.

use crate::parser::tree::api::Ast;
use crate::utils::display;

/// Brace-block node, starting with `{` and ending with `}`.
///
/// The whole code of a file is also considered a block, with `full` always
/// `false`. This allows use to pushed blocks with no relations, like a
/// succession of functions.
#[derive(Debug, Default)]
pub struct BracedBlock {
    /// Elements of the braced-block, separated by `;`.
    pub elts: Vec<Ast>,
    /// indicates whether the closing `}` for the arguments was found or
    /// not
    ///
    /// If `full` is `false`, we can still push blocks inside.
    pub full: bool,
}

display!(
    BracedBlock,
    self,
    f,
    write!(
        f,
        "[{}{}]",
        self.elts
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<_>>()
            .join(", "),
        if self.full { "" } else { ".." }
    )
);
