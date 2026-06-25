//! Module that defines the main node types of the [`Ast`]

use crate::BracedBlock;
use crate::errors::api::ErrorLocation;
use crate::parser::tree::api::Ast;
use crate::parser::variable::Variable;
use crate::utils::{display, repr_vec};

/// Function call
///
/// This node represents function calls, function declarations and function
/// definitions.
///
/// In the case of function calls, the variable should be a line name without
/// attribute, and the body should be empty.
///
/// In the case of function declarations, the variable should be a variable
/// declaration with attribute, and the
/// body should be empty.
///
/// In the case of function definitions, the variable should be a variable
/// declaration with attribute, and the body should be a [`BracedBlock`].
#[derive(Debug)]
pub struct FunctionCall {
    /// arguments passed to the function
    pub arguments: Vec<Ast>,
    /// body of the function if it is a definition
    pub function_body: Option<BracedBlock>,
    /// name of the function, and all its attributes (return type)
    pub variable: Variable,
}

display!(
    FunctionCall,
    self,
    f,
    if let Some(body) = &self.function_body {
        write!(f, "({}\u{b0}({}){body})", self.variable, repr_vec(&self.arguments, ", "))
    } else {
        write!(f, "({}\u{b0}({}))", self.variable, repr_vec(&self.arguments, ", "))
    }
);

/// List initialiser
///
/// Node to represent list initialisers, such as `{1, 2, 3, [6]=12}`.
#[derive(Debug)]
pub struct ListInitialiser {
    /// elements of the list
    pub elts: Vec<Ast>,
    /// indicates whether the closing `}` was found yet.
    ///
    /// If full is false, we can still push elements inside.
    pub full: bool,
    /// Location of the entire list.
    pub location: ErrorLocation,
}

display!(
    ListInitialiser,
    self,
    f,
    write!(
        f,
        "{{{}}}",
        self.elts
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<_>>()
            .join(", ")
    )
);
