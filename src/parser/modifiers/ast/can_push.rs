//! Methods to test if push a specific token into an [`Ast`] is possible

use crate::parser::keyword::control_flow::node::ControlFlowNode;
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::types::binary::Binary;
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::literal::Attribute;
use crate::parser::types::parens::Cast;
use crate::parser::types::ternary::Ternary;
use crate::parser::types::unary::Unary;
use crate::parser::types::variable::traits::PureType as _;
use crate::parser::types::{Ast, ListInitialiser};

impl CanPush for Ast {
    fn can_push_leaf_with_ctx(&self, ctx: AstPushContext) -> bool {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::custom_print(&format!(
            "Can push leaf in {self} with ctx {ctx:?}"
        ));
        match self {
            Self::Empty
            | Self::Cast(Cast { full: true, .. })
            | Self::Ternary(Ternary { failure: None, .. }) => true,
            Self::Variable(var) => ctx.is_user_variable() || var.can_push_leaf(),
            Self::ParensBlock(parens) => parens.is_pure_type() && ctx.is_user_variable(),
            Self::Leaf(_) | Self::FunctionCall(_) => false,
            Self::Cast(Cast {
                full: false,
                value: arg,
                ..
            })
            | Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(Ternary {
                failure: Some(arg), ..
            }) => arg.can_push_leaf_with_ctx(ctx),
            Self::FunctionArgsBuild(vec) => vec
                .last()
                .is_none_or(|child| child.can_push_leaf_with_ctx(ctx)),
            Self::BracedBlock(BracedBlock { elts: vec, full })
            | Self::ListInitialiser(ListInitialiser { full, elts: vec }) => {
                !*full
                    && vec
                        .last()
                        .is_none_or(|child| child.can_push_leaf_with_ctx(ctx))
            }
            // Full not complete because: `if (0) 1; else 2;`
            Self::ControlFlow(ctrl) if ctx.is_else() => {
                if let ControlFlowNode::Condition(cond) = ctrl
                    && cond.no_else()
                {
                    true
                } else {
                    ctrl.as_ast()
                        .is_some_and(|ast| ast.can_push_leaf_with_ctx(ctx))
                }
            }
            // Complete not full because: `if (0) 1; 2;`
            Self::ControlFlow(ctrl) => !ctrl.is_complete(),
        }
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
    const fn is_else(&self) -> bool {
        matches!(self, &Self::Any | &Self::Else)
    }

    /// Checks if the context can have a variable
    const fn is_user_variable(&self) -> bool {
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
    fn can_push_leaf(&self) -> bool {
        self.can_push_leaf_with_ctx(AstPushContext::None)
    }
    /// See [`CanPush`] documentation.
    fn can_push_leaf_with_ctx(&self, _ctx: AstPushContext) -> bool {
        self.can_push_leaf()
    }
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
