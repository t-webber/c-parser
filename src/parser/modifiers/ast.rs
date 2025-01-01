//! Implements the method for pushing in and looking at an [`Ast`].

use core::cmp::Ordering;
use core::{fmt, mem};

use super::super::types::binary::Binary;
use super::super::types::blocks::BracedBlock;
use super::super::types::literal::{Attribute, Literal, Variable, VariableName};
use super::super::types::operator::{Associativity, Operator as _};
use super::super::types::unary::Unary;
use super::super::types::{FunctionCall, ListInitialiser};
use super::conversions::OperatorConversions;
use crate::EMPTY;
use crate::parser::types::Ast;
use crate::parser::types::ternary::Ternary;

impl Ast {
    /// Finds the leaf the most left possible, checks it is a variable and
    /// pushes it some attributes.
    pub fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Attribute>,
    ) -> Result<(), String> {
        let make_error = |msg: &str| Err(format!("LHS: {msg} are illegal in type declarations."));

        match self {
            Self::Empty => Err("LHS: Missing argument.".to_owned()),
            Self::Leaf(Literal::Variable(Variable { attrs, .. })) => {
                let old_attrs = mem::take(attrs);
                attrs.reserve(previous_attrs.len().checked_add(attrs.len()).ok_or_else(|| "Code overflow occurred. Please reduce the number of attributes applied to this variable.".to_owned())?);
                attrs.extend(previous_attrs);
                attrs.extend(old_attrs);
                Ok(())
            }
            Self::Leaf(_) => make_error("constant"),
            Self::ParensBlock(_) => make_error("parenthesis"),
            Self::Unary(Unary { arg, .. }) | Self::Binary(Binary { arg_l: arg, .. }) => {
                arg.add_attribute_to_left_variable(previous_attrs)
            }
            Self::Ternary(Ternary { condition, .. }) => {
                condition.add_attribute_to_left_variable(previous_attrs)
            }
            Self::FunctionCall(_) => make_error("Functions"),
            Self::ListInitialiser(_) => make_error("List initialisers"),
            Self::BracedBlock(_) => make_error("Blocks"),
            Self::ControlFlow(_) => make_error("Control flow keywords"),
        }
    }

    /// Checks if a [`Ast`] is pushable
    ///
    /// # Returns
    ///  - `false` if one child on the right branch is empty
    ///  - `true` otherwise
    fn can_push_leaf(&self, is_user_variable: bool) -> bool {
        match self {
            Self::Empty | Self::Ternary(Ternary { failure: None, .. }) => true,
            Self::Leaf(Literal::Variable(_)) => is_user_variable,
            Self::Leaf(_) | Self::ParensBlock(_) => false,
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(Ternary {
                failure: Some(arg), ..
            }) => arg.can_push_leaf(is_user_variable),
            Self::BracedBlock(BracedBlock { elts: vec, full })
            | Self::ListInitialiser(ListInitialiser { full, elts: vec })
            | Self::FunctionCall(FunctionCall {
                full, args: vec, ..
            }) => {
                !*full
                    && vec
                        .last()
                        .is_none_or(|child| child.can_push_leaf(is_user_variable))
            }
            Self::ControlFlow(ctrl) => !ctrl.is_full(),
        }
    }

    /// Pushes a node at the bottom of the [`Ast`].
    ///
    /// This methods considers `node` as a leaf, and pushes it as a leaf into
    /// the [`Ast`].
    pub fn push_block_as_leaf(&mut self, node: Self) -> Result<(), String> {
        match self {
            //
            //
            // success
            Self::Empty => {
                *self = node;
                Ok(())
            }
            //
            //
            // full: ok, but create a new block
            // Example: {a}b
            Self::BracedBlock(BracedBlock { full: true, .. }) => {
                *self = Self::BracedBlock(BracedBlock {
                    elts: vec![mem::take(self), node],
                    full: false,
                });
                Ok(())
            }
            //
            //
            // previous is incomplete variable: waiting for variable name
            Self::Leaf(Literal::Variable(var)) => {
                let err = format!("{node}");
                if let Self::Leaf(Literal::Variable(Variable { attrs, name })) = node
                    && attrs.is_empty()
                {
                    var.push_name(name)
                } else {
                    Err(format!(
                        "Expected variable name after attribute keywords, but found {err}"
                    ))
                }
            }

            //
            //
            // atomic: failure
            Self::ParensBlock(old) => Err(successive_literal_error("Parenthesis group", old, node)),
            Self::Leaf(old) => Err(successive_literal_error("Literal", old, node)),
            //
            //
            // full: failure
            Self::FunctionCall(FunctionCall { full: true, .. }) => {
                Err(successive_literal_error("Function call", self, node))
            }
            Self::ListInitialiser(ListInitialiser { full: true, .. }) => {
                Err(successive_literal_error("List initialiser", self, node))
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
            ) => arg.push_block_as_leaf(node),
            // lists
            Self::ListInitialiser(ListInitialiser {
                elts: vec,
                full: false,
            })
            | Self::FunctionCall(FunctionCall {
                args: vec,
                full: false,
                ..
            })
            | Self::BracedBlock(BracedBlock {
                elts: vec,
                full: false,
            }) => {
                if let Some(last) = vec.last_mut() {
                    if last.can_push_leaf(matches!(
                        node,
                        Self::Leaf(Literal::Variable(Variable {
                            name: VariableName::UserDefined(_),
                            ..
                        }))
                    )) {
                        last.push_block_as_leaf(node)
                    } else if matches!(last, Self::BracedBlock(_)) {
                        // Example: {{a}b}
                        vec.push(node);
                        Ok(())
                    } else {
                        // Example: {a b}
                        Err(successive_literal_error("block", self, node))
                    }
                } else {
                    *vec = vec![node];
                    Ok(())
                }
            }
            Self::ControlFlow(ctrl) => ctrl.push_block_as_leaf(node),
        }
    }

    /// Adds a braced block to the [`Ast`]
    pub fn push_braced_block(&mut self, braced_block: Self) {
        let mut node = braced_block;
        if let Self::BracedBlock(BracedBlock { full, .. }) = &mut node {
            *full = true;
        } else {
            panic!("a block can't be changed to another node")
        }
        #[expect(clippy::wildcard_enum_match_arm)]
        match self {
            Self::BracedBlock(BracedBlock { elts, full }) if !*full => elts.push(node),
            Self::Empty => *self = node,
            _ => {
                *self = Self::BracedBlock(BracedBlock {
                    elts: vec![mem::take(self), node],
                    full: false,
                });
            }
        }
    }

    /// Tries to push an operator in the [`Ast`]
    ///
    /// This method finds, with the associativities, precedences and arities,
    /// were to push the `op` into the [`Ast`].
    pub fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display,
    {
        match self {
            //
            //
            // self empty: Self = Op
            Self::Empty => op.try_convert_and_erase_node(self),
            //
            //
            // self is a non-modifiable block: Op -> Self
            Self::ListInitialiser(ListInitialiser { full: true, .. })
            | Self::FunctionCall(FunctionCall { full: true, .. })
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
            Self::FunctionCall(FunctionCall {
                args: vec,
                full: false,
                ..
            })
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
                    *vec = vec![op.try_to_node()?];
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
            Self::ControlFlow(_) => Err(format!(
                "Illegal operator {op} in this context: unfinished control flow."
            )),
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => EMPTY.fmt(f),
            Self::Binary(val) => val.fmt(f),
            Self::FunctionCall(val) => val.fmt(f),
            Self::Leaf(val) => val.fmt(f),
            Self::Ternary(val) => val.fmt(f),
            Self::Unary(val) => val.fmt(f),
            Self::BracedBlock(block) => block.fmt(f),
            Self::ListInitialiser(list_initialiser) => list_initialiser.fmt(f),
            Self::ParensBlock(parens) => parens.fmt(f),
            Self::ControlFlow(ctrl) => ctrl.fmt(f),
        }
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
