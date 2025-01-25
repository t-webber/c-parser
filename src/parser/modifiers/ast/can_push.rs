//! Methods to test if push a specific token into an [`Ast`] is possible

use crate::parser::types::Ast;
use crate::parser::types::binary::Binary;
use crate::parser::types::literal::Attribute;
use crate::parser::types::ternary::Ternary;
use crate::parser::types::unary::Unary;

impl CanPush for Ast {
    fn can_push_leaf(&self) -> bool {
        self.can_push_leaf_with_ctx(AstPushContext::None)
    }
}

impl PushAttribute for Ast {
    fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Attribute>,
    ) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::custom_print(&format!(
            "\tAdding attrs {} to ast {self}",
            crate::parser::repr_vec(&previous_attrs)
        ));
        let make_error = |msg: &str| Err(format!("LHS: {msg} are illegal in type declarations."));
        match self {
            Self::Empty => Err("LHS: Missing argument.".to_owned()),
            Self::Variable(var) => var.add_attribute_to_left_variable(previous_attrs),
            Self::Leaf(_) => make_error("constant"),
            Self::ParensBlock(_) => make_error("parenthesis"),
            Self::Unary(Unary { arg, .. }) | Self::Binary(Binary { arg_l: arg, .. }) => {
                arg.add_attribute_to_left_variable(previous_attrs)
            }
            Self::Ternary(Ternary { condition, .. }) => {
                condition.add_attribute_to_left_variable(previous_attrs)
            }
            Self::Cast(_) => make_error("Casts"),
            Self::FunctionArgsBuild(_) => make_error("Functions arguments"),
            Self::FunctionCall(_) => make_error("Functions"),
            Self::ListInitialiser(_) => make_error("List initialisers"),
            Self::BracedBlock(_) => make_error("Blocks"),
            Self::ControlFlow(_) => make_error("Control flow keywords"),
        }
    }
}

/// Context to specify what are we trying to push into the [`Ast`].
///
/// See [`Ast::can_push_leaf`] for more information.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum AstPushContext {
    /// Any context is good
    Any,
    /// Trying to see if an `else` block ca be added
    Else,
    /// Nothing particular
    #[default]
    None,
    /// Trying to see if the last element of the [`Ast`] waiting for variables.
    UserVariable,
}

impl AstPushContext {
    /// Checks if the context can have an `else`
    pub const fn is_else(&self) -> bool {
        matches!(self, &Self::Any | &Self::Else)
    }

    /// Checks if the context can have a variable
    pub const fn is_user_variable(&self) -> bool {
        matches!(self, &Self::Any | &Self::UserVariable)
    }
}

/// Methods to check if a node is pushable
///
/// # Note
///
/// Whether an [`Ast`] is pushable or not sometimes depends on what it is we
/// are trying to push. This is goal of the [`AstPushContext`].
///
/// # Examples
///
/// When pushing an `else` keyword into an
/// [`ControlFlowNode`],
/// the latter is pushable iff the control flow is complete (not
/// necessary full)! But when pushing a literal into a
/// [`ControlFlowNode`],
/// the latter is pushable iff the control flow is full (not only
/// complete). See
/// [`ControlFlowNode::is_full`]
/// and
/// [`ControlFlowNode::is_complete`] to see the difference.
pub trait CanPush {
    /// See [`CanPush`] documentation.
    fn can_push_leaf(&self) -> bool;
}

/// Finds the leaf the most left possible, checks it is a variable and
/// pushes it some attributes.
///
/// This is used when finding an identifier or keyword after a full [`Ast`],
/// where it is meant as a type declaration.
pub trait PushAttribute {
    /// See [`PushAttribute`].
    fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Attribute>,
    ) -> Result<(), String>;
}
