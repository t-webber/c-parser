//! Defines the brace-block nodes.

use crate::errors::api::ErrorLocation;
use crate::parser::tree::api::Ast;
use crate::utils::display;

/// Brace-block node, starting with `{` and ending with `}`.
///
/// The whole code of a file is also considered a block, with `full` always
/// `false`. This allows use to pushed blocks with no relations, like a
/// succession of functions.
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct BracedBlock {
    /// Elements of the braced-block, separated by `;`.
    pub elts: Vec<Ast>,
    /// indicates whether the closing `}` for the arguments was found or
    /// not
    ///
    /// If `full` is `false`, we can still push blocks inside.
    pub full: bool,
    /// Location of the braced block.
    pub location: ErrorLocation,
}

impl BracedBlock {
    /// Creates a braced block from an ast, moving it to the first place and
    /// making it pushable.
    #[must_use]
    pub fn from_node(ast: Ast) -> Self {
        let location = ast.location();
        Self { elts: vec![ast, Ast::Empty], full: false, location }
    }
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
