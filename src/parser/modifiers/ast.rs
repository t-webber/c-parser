//! Implements the method for pushing in and looking at an [`Ast`].

use core::cmp::Ordering;
use core::{fmt, mem};

use super::conversions::OperatorConversions;
use super::push::Push;
use crate::EMPTY;
use crate::parser::keyword::control_flow::node::ControlFlowNode;
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::repr_vec;
use crate::parser::types::binary::{Binary, BinaryOperator};
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::literal::Attribute;
use crate::parser::types::operator::{Associativity, Operator as _};
use crate::parser::types::ternary::Ternary;
use crate::parser::types::unary::{Unary, UnaryOperator};
use crate::parser::types::{Ast, ListInitialiser};

impl Ast {
    /// Finds the leaf the most left possible, checks it is a variable and
    /// pushes it some attributes.
    pub fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Attribute>,
    ) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::custom_print(&format!(
            "\tAdding attrs {} to ast {self}",
            repr_vec(&previous_attrs)
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
            Self::FunctionArgsBuild(_) => make_error("Functions arguments"),
            Self::FunctionCall(_) => make_error("Functions"),
            Self::ListInitialiser(_) => make_error("List initialisers"),
            Self::BracedBlock(_) => make_error("Blocks"),
            Self::ControlFlow(_) => make_error("Control flow keywords"),
        }
    }

    /// Checks if a [`Ast`] is pushable
    ///
    /// # Returns
    ///
    ///  - `false` if one child on the right branch is empty
    ///  - `true` otherwise
    pub fn can_push_leaf(&self) -> bool {
        self.can_push_leaf_with_ctx(AstPushContext::None)
    }

    /// Checks if a [`Ast`] is pushable
    ///
    /// # Returns
    ///
    ///  - `false` if one child on the right branch is empty
    ///  - `true` otherwise
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
    pub fn can_push_leaf_with_ctx(&self, ctx: AstPushContext) -> bool {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::custom_print(&format!(
            "Can push leaf in {self} with ctx {ctx:?}"
        ));
        match self {
            Self::Empty | Self::Ternary(Ternary { failure: None, .. }) => true,
            Self::Variable(_) => ctx.is_user_variable(),
            Self::Leaf(_) | Self::ParensBlock(_) | Self::FunctionCall(_) => false,
            Self::Unary(Unary { arg, .. })
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
                    ctrl.get_ast()
                        .is_some_and(|ast| ast.can_push_leaf_with_ctx(ctx))
                }
            }
            // Complete not full because: `if (0) 1; 2;`
            Self::ControlFlow(ctrl) => !ctrl.is_complete(),
        }
    }

    /// Creates an empty [`Ast`] inside a [`Box`] to initialise nodes
    pub fn empty_box() -> Box<Self> {
        Box::new(Self::Empty)
    }

    /// Marks the [`Ast`] as full.
    pub fn fill(&mut self) {
        match self {
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(
                Ternary {
                    failure: Some(arg), ..
                }
                | Ternary { success: arg, .. },
            ) => arg.fill(),
            Self::ControlFlow(ctrl) => ctrl.fill(),
            Self::BracedBlock(BracedBlock { full, .. })
            | Self::ListInitialiser(ListInitialiser { full, .. }) => *full = true,
            Self::Variable(var) => var.fill(),
            Self::FunctionCall(_)
            | Self::FunctionArgsBuild(_)
            | Self::Empty
            | Self::Leaf(_)
            | Self::ParensBlock(_) => (),
        }
    }

    /// Checks if the last element is a leaf, and another attribute/name can be
    /// pushed.
    fn is_in_leaf_ctx(&self, is_leaf: bool) -> bool {
        match self {
            Self::Variable(var) => is_leaf || var.is_declaration(),
            Self::Binary(Binary {
                op: BinaryOperator::Multiply,
                arg_r: arg,
                ..
            })
            | Self::Unary(Unary {
                arg,
                op: UnaryOperator::Indirection,
            }) => arg.is_in_leaf_ctx(is_leaf),

            Self::BracedBlock(BracedBlock { elts, full: false }) => {
                elts.last().is_some_and(|last| last.is_in_leaf_ctx(is_leaf))
            }
            Self::ControlFlow(ctrl) if !ctrl.is_full() => ctrl
                .get_ast()
                .is_some_and(|last| last.is_in_leaf_ctx(is_leaf)),
            Self::Empty
            | Self::Leaf(_)
            | Self::Unary(_)
            | Self::Binary(_)
            | Self::Ternary(_)
            | Self::ParensBlock(_)
            | Self::ControlFlow(_)
            | Self::BracedBlock(_)
            | Self::FunctionCall(_)
            | Self::ListInitialiser(_)
            | Self::FunctionArgsBuild(_) => false,
        }
    }

    /// Push an [`Ast`] as leaf into a vector [`Ast`].
    ///
    /// This is a wrapper for [`Ast::push_block_as_leaf`].
    fn push_block_as_leaf_in_vec(vec: &mut Vec<Self>, node: Self) -> Result<Option<Self>, String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_in_vec(&node, vec, "vec");
        if let Some(last) = vec.last_mut() {
            let ctx = if last.is_in_leaf_ctx(matches!(node, Self::Variable(_))) {
                AstPushContext::UserVariable
            } else {
                AstPushContext::None
            };
            if last.can_push_leaf_with_ctx(ctx) {
                last.push_block_as_leaf(node)?;
            } else if matches!(last, Self::BracedBlock(_)) {
                // Example: `{{a}b}`
                vec.push(node);
            } else if let Self::ControlFlow(ctrl) = last
                && ctrl.is_complete()
            {
                // Example `if(a) {return x} b`
                vec.push(node);
            } else {
                // Example: {a b}
                // Error
                return Ok(Some(node));
            }
        } else {
            vec.push(node);
        }
        Ok(None)
    }

    /// Pushes a [`BracedBlock`] into an [`Ast`]
    pub fn push_braced_block(&mut self, braced_block: Self) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf_in(&braced_block, "braced", self, "ast");
        match self {
            Self::BracedBlock(BracedBlock { elts, full: false }) => {
                if let Some(last_mut) = elts.last_mut() {
                    if let Self::ControlFlow(ctrl) = last_mut
                        && !ctrl.is_full()
                    {
                        ctrl.push_block_as_leaf(braced_block)?;
                    } else if let Self::Variable(var) = last_mut
                        && let Some((keyword, name)) = var.get_partial_typedef()
                    {
                        if let Self::BracedBlock(block) = braced_block {
                            *last_mut =
                                Self::ControlFlow(keyword.to_control_flow(name, Some(block)));
                        } else {
                            panic!("see above: still block")
                        }
                    } else {
                        elts.push(braced_block);
                    }
                } else {
                    elts.push(braced_block);
                }
            }
            Self::ControlFlow(ctrl) if !ctrl.is_full() => ctrl.push_block_as_leaf(braced_block)?,
            Self::Empty => *self = braced_block,
            Self::Leaf(_)
            | Self::Unary(_)
            | Self::Binary(_)
            | Self::Ternary(_)
            | Self::Variable(_)
            | Self::ParensBlock(_)
            | Self::BracedBlock(_)
            | Self::ControlFlow(_)
            | Self::FunctionCall(_)
            | Self::ListInitialiser(_)
            | Self::FunctionArgsBuild(_) => {
                panic!("Trying to push block {braced_block} in {self}")
            }
        }
        Ok(())
    }
}

impl Push for Ast {
    fn push_block_as_leaf(&mut self, ast: Self) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "ast");
        match self {
            //
            //
            // success
            Self::Empty => {
                *self = ast;
                Ok(())
            }
            //
            //
            // full: ok, but create a new block
            // Example: {a}b
            Self::BracedBlock(BracedBlock { full: true, .. }) => {
                *self = Self::BracedBlock(BracedBlock {
                    elts: vec![mem::take(self), ast],
                    full: false,
                });
                Ok(())
            }
            //
            //
            // previous is incomplete variable: waiting for variable name
            Self::Variable(var) => var.push_block_as_leaf(ast),

            //
            //
            // atomic: failure
            Self::ParensBlock(old) => Err(successive_literal_error("Parenthesis group", old, ast)),
            Self::Leaf(old) => Err(successive_literal_error("Literal", old, ast)),
            //
            //
            // full: failure
            Self::FunctionCall(_) => Err(successive_literal_error("Function call", self, ast)),
            Self::ListInitialiser(ListInitialiser { full: true, .. }) => {
                Err(successive_literal_error("List initialiser", self, ast))
            }
            //
            //
            // recurse
            // operators
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(
                Ternary {
                    failure: Some(arg), ..
                }
                | Ternary { success: arg, .. },
            ) => arg.push_block_as_leaf(ast),
            // lists
            Self::FunctionArgsBuild(vec)
            | Self::ListInitialiser(ListInitialiser {
                elts: vec,
                full: false,
            })
            | Self::BracedBlock(BracedBlock {
                elts: vec,
                full: false,
            }) => (Self::push_block_as_leaf_in_vec(vec, ast)?).map_or(Ok(()), |err_node| {
                Err(successive_literal_error("block", self, err_node))
            }),
            Self::ControlFlow(ctrl) if ctrl.is_complete() => {
                *self = Self::BracedBlock(BracedBlock {
                    elts: vec![mem::take(self), ast],
                    full: false,
                });
                Ok(())
            }
            Self::ControlFlow(ctrl) => ctrl.push_block_as_leaf(ast),
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "ast");
        match self {
            Self::Empty => op.try_convert_and_erase_node(self),
            Self::Variable(var) => {
                if !var.is_full() && var.is_declaration() && !op.is_array_subscript() {
                    var.push_op(op)
                } else {
                    op.try_push_op_as_root(self)
                }
            }
            //
            //
            // self is a non-modifiable block: Op -> Self
            Self::ListInitialiser(ListInitialiser { full: true, .. })
            | Self::FunctionCall(_)
            | Self::Leaf(_)
            | Self::ParensBlock(_) => op.try_push_op_as_root(self),
            //
            //
            // full block: make space: Self = [Self, Empty]
            Self::BracedBlock(BracedBlock { full: true, .. }) => {
                *self = Self::BracedBlock(BracedBlock {
                    elts: vec![mem::take(self), Self::Empty],
                    full: false,
                });
                self.push_op(op)
            }
            //
            //
            // pushable list: self.last.push_op(op)
            Self::FunctionArgsBuild(vec)
            | Self::BracedBlock(BracedBlock {
                elts: vec,
                full: false,
            })
            | Self::ListInitialiser(ListInitialiser {
                elts: vec,
                full: false,
            }) => {
                if let Some(last) = vec.last_mut() {
                    last.push_op(op)
                } else {
                    vec.push(op.try_to_node()?);
                    Ok(())
                }
            }
            //
            //
            // operators
            Self::Unary(Unary { op: old_op, arg }) => {
                match old_op.precedence().cmp(&op.precedence()) {
                    Ordering::Less => op.try_push_op_as_root(self),
                    // doing whatever works for [`Ordering::Equal`] ? no ! e.g.: !g(!x) gives !!g(x)
                    // for `op.try_push_op_as_root(self)`
                    Ordering::Greater | Ordering::Equal => arg.push_op(op),
                }
            }
            Self::Binary(Binary {
                op: old_op,
                arg_r: arg,
                ..
            }) => {
                let associativity = op.associativity(); // same associativity for same precedence
                match (old_op.precedence().cmp(&op.precedence()), associativity) {
                    (Ordering::Less, _) | (Ordering::Equal, Associativity::LeftToRight) => {
                        op.try_push_op_as_root(self)
                    }
                    (Ordering::Greater, _) | (Ordering::Equal, Associativity::RightToLeft) => {
                        arg.push_op(op)
                    }
                }
            }
            Self::Ternary(Ternary {
                op: old_op,
                failure: Some(arg),
                ..
            }) => {
                let associativity = op.associativity(); // same associativity for same precedence
                match (old_op.precedence().cmp(&op.precedence()), associativity) {
                    (Ordering::Less, _) | (Ordering::Equal, Associativity::LeftToRight) => {
                        op.try_push_op_as_root(self)
                    }
                    (Ordering::Greater, _) | (Ordering::Equal, Associativity::RightToLeft) => {
                        arg.push_op(op)
                    }
                }
            }
            //
            //
            // explicit derogation clause on success block of a ternary operator
            Self::Ternary(Ternary { success: arg, .. }) => arg.push_op(op),
            //
            //
            // Control flows
            Self::ControlFlow(ctrl) => ctrl.push_op(op),
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => EMPTY.fmt(f),
            Self::Unary(val) => val.fmt(f),
            Self::Leaf(val) => val.fmt(f),
            Self::Binary(val) => val.fmt(f),
            Self::Ternary(val) => val.fmt(f),
            Self::Variable(var) => var.fmt(f),
            Self::FunctionCall(val) => val.fmt(f),
            Self::BracedBlock(block) => block.fmt(f),
            Self::ParensBlock(parens) => parens.fmt(f),
            Self::ControlFlow(ctrl) => ctrl.fmt(f),
            Self::FunctionArgsBuild(vec) => write!(f, "(\u{b0}{})", repr_vec(vec)),
            Self::ListInitialiser(list_initialiser) => list_initialiser.fmt(f),
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
    #[inline]
    const fn is_else(&self) -> bool {
        matches!(self, &Self::Any | &Self::Else)
    }

    /// Checks if the context can have a variable
    #[inline]
    const fn is_user_variable(&self) -> bool {
        matches!(self, &Self::Any | &Self::UserVariable)
    }
}

/// Makes an error [`String`] for consecutive literals.
///
/// If two consecutive literals are found, the [`crate::parser`] fails, and this
/// is the generic function to make the uniformed-string-value-error.
fn successive_literal_error<T: fmt::Display, U: fmt::Display>(
    old_type: &str,
    old: T,
    new: U,
) -> String {
    format!("Found 2 consecutive literals: {old_type} {old} followed by {new}.")
}
