//! Defines the brace-block nodes.

use core::fmt;

use super::Ast;

/// Brace-block node, starting with `{` and ending with `}`.
///
/// The whole code of a file is also considered a block, with `full` always
/// `false`. This allows use to pushed blocks with no relations, like a
/// succession of functions.
#[derive(Debug, Default, PartialEq)]
pub struct BracedBlock {
    /// Elements of the braced-block, separated by `;`.
    pub elts: Vec<Ast>,
    /// indicates whether the closing `}` for the arguments was found or
    /// not
    ///
    /// If `full` is `false`, we can still push blocks inside.
    pub full: bool,
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for BracedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    }
}
