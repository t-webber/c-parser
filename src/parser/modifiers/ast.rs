//! Implements the method for pushing in and looking at an [`Ast`].

use core::cmp::Ordering;
use core::{fmt, mem};

use super::conversions::OperatorConversions;
use super::push::Push;
use crate::EMPTY;
use crate::parser::repr_vec;
use crate::parser::types::binary::Binary;
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::literal::Attribute;
use crate::parser::types::operator::{Associativity, Operator as _};
use crate::parser::types::ternary::Ternary;
use crate::parser::types::unary::Unary;
use crate::parser::types::{Ast, ListInitialiser};

impl Ast {
    /// Finds the leaf the most left possible, checks it is a variable and
    /// pushes it some attributes.
    pub fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Attribute>,
    ) -> Result<(), String> {
        #[cfg(feature = "debug")]
        println!("\tAdding attrs {} to ast {self}", repr_vec(&previous_attrs));
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
    /// [`ControlFlowNode`](crate::parser::keyword::control_flow::node::ControlFlowNode),
    /// the latter is pushable iff the control flow is complete (not
    /// necessary full)! But when pushing a literal into a
    /// [`ControlFlowNode`](crate::parser::keyword::control_flow::node::ControlFlowNode),
    /// the latter is pushable iff the control flow is full (not only
    /// complete). See
    /// [`ControlFlowNode::is_full`](crate::parser::keyword::control_flow::node::ControlFlowNode::is_full)
    /// and
    /// [`ControlFlowNode::is_complete`](crate::parser::keyword::control_flow::node::ControlFlowNode::is_complete) to see the difference.
    pub fn can_push_leaf_with_ctx(&self, ctx: AstPushContext) -> bool {
        match self {
            Self::Empty | Self::Ternary(Ternary { failure: None, .. }) => true,
            Self::Variable(_) => ctx == AstPushContext::UserVariable,
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
            Self::ControlFlow(ctrl) if ctx == AstPushContext::Else => !ctrl.is_full(),
            // Complete not full because: `if (0) 1; 2;`
            Self::ControlFlow(ctrl) => !ctrl.is_complete(),
        }
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

    /// Push an [`Ast`] as leaf into a vector [`Ast`].
    ///
    /// This is a wrapper for [`Ast::push_block_as_leaf`].
    fn push_block_as_leaf_in_vec(vec: &mut Vec<Self>, node: Self) -> Result<Option<Self>, String> {
        #[cfg(feature = "debug")]
        println!("\tPushing {node} as leaf in vec {}", repr_vec(vec),);
        if let Some(last) = vec.last_mut() {
            let ctx = if let Self::Variable(_) = last
            // && var.is_declaration() // needs example // uncommented for TYPE x
            {
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
        println!("\tPushing braced {braced_block} in ast {self}");
        #[expect(clippy::wildcard_enum_match_arm)]
        match self {
            Self::BracedBlock(BracedBlock { elts, full: false }) => {
                let last_mut = elts.last_mut();
                if let Some(Self::ControlFlow(ctrl)) = last_mut
                    && !ctrl.is_full()
                {
                    ctrl.push_block_as_leaf(braced_block)?;
                } else if let Some(Self::Variable(var)) = last_mut
                    && let Some((keyword, name)) = var.get_typedef()
                {
                    if let Self::BracedBlock(block) = braced_block {
                        *self = Self::ControlFlow(keyword.to_control_flow(name, block));
                    } else {
                        panic!("see above: still block")
                    }
                } else {
                    elts.push(braced_block);
                }
            }
            Self::ControlFlow(ctrl) if !ctrl.is_full() => ctrl.push_block_as_leaf(braced_block)?,
            Self::Empty => *self = braced_block,
            _ => {
                panic!("Trying to push block {braced_block} in {self}")
            }
        }
        Ok(())
    }
}

impl Push for Ast {
    fn push_block_as_leaf(&mut self, ast: Self) -> Result<(), String> {
        #[cfg(feature = "debug")]
        println!("\tPushing {ast} as leaf in ast {self}");
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
            Self::ControlFlow(ctrl) if ctrl.is_full() => panic!("never push on full control"),
            Self::ControlFlow(ctrl) => ctrl.push_block_as_leaf(ast),
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        println!("\tPushing op {op} in ast {self}");
        match self {
            Self::Empty => op.try_convert_and_erase_node(self),
            Self::Variable(var) => {
                if var.push_op(op).is_err() {
                    op.try_push_op_as_root(self)
                } else {
                    Ok(())
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
            Self::FunctionArgsBuild(vec) => write!(f, "({})", repr_vec(vec)),
            Self::ListInitialiser(list_initialiser) => list_initialiser.fmt(f),
        }
    }
}

/// Context to specify what are we trying to push into the [`Ast`].
///
/// See [`Ast::can_push_leaf`] for more information.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum AstPushContext {
    /// Trying to see if an `else` block ca be added
    Else,
    /// Nothing particular
    #[default]
    None,
    /// Trying to see if the last element of the [`Ast`] waiting for variables.
    UserVariable,
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
