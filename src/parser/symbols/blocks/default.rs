//! Module that defines the main node types of the [`Ast`]

use core::fmt;

use crate::parser::display::repr_vec;
use crate::parser::tree::api::Ast;
use crate::parser::variable::Variable;

/// Function call
///
/// This node represents functions declaration, functions
#[derive(Debug)]
pub struct FunctionCall {
    /// arguments of the function
    pub args: Vec<Ast>,
    /// name of the function, and all its attributes (return type)
    pub variable: Variable,
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}\u{b0}({}))", self.variable, repr_vec(&self.args),)
    }
}

/// List initialiser
///
/// Node to represent list initialisers, such as `{1, 2, 3, [6]=12}`.
#[derive(Debug, Default)]
pub struct ListInitialiser {
    /// elements of the list
    pub elts: Vec<Ast>,
    /// indicates whether the closing `}` was found yet.
    ///
    /// If full is false, we can still push elements inside.
    pub full: bool,
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for ListInitialiser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.elts
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
