use core::cmp::Ordering;
use core::{fmt, mem};

use super::binary::{Binary, BinaryOperator};
use super::blocks::Block;
use super::conversions::OperatorConversions;
use super::traits::{Associativity, IsComma, Operator as _};
use super::unary::Unary;
use super::{
    FunctionCall, FunctionOperator, ListInitialiser, Literal, Ternary, Variable, VariableName
};
use crate::EMPTY;

/// Struct to represent the AST
#[expect(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Default, PartialEq)]
pub enum Ast {
    // atomic
    #[default]
    Empty,
    Leaf(Literal),
    // operators
    Unary(Unary),
    Binary(Binary),
    Ternary(Ternary),
    // non atomic leafs
    FunctionCall(FunctionCall),
    ListInitialiser(ListInitialiser),
    // block
    Block(Block),
    // parenthesis
    ParensBlock(Box<Ast>),
    // TODO: while, for, goto, etc; CompoundLiteral(CompoundLiteral),; SpecialUnary(SpecialUnary),
}

impl Ast {
    /// Applies a closure to the current [`ListInitialiser`].
    ///
    /// It applies the closure somewhere in the [`Ast`]. If this closure
    /// returns a value, it is returns in `Ok(_)`. If no list initialiser is
    /// found, `Err(())` is returned.
    ///
    /// In the case of nested [`ListInitialiser`]s, the closure is applied to
    /// the one closest from the leaves.
    #[expect(clippy::min_ident_chars)]
    pub fn apply_to_last_list_initialiser<T, F: Fn(&mut Vec<Self>, &mut bool) -> T>(
        &mut self,
        f: &F,
    ) -> Result<T, ()> {
        match self {
            //
            //
            // success
            Self::ListInitialiser(ListInitialiser {
                elts,
                full: full @ false,
            }) => {
                if let Some(last) = elts.last_mut() {
                    if let res @ Ok(_) = last.apply_to_last_list_initialiser(f) {
                        return res;
                    }
                }
                Ok(f(elts, full))
            }
            //
            //
            // failure
            // atomic
            Self::Empty
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            // child is none
            | Self::Unary(Unary{arg:None, ..})
            | Self::Binary(Binary{arg_r:None, ..})
            // full lists
            | Self::Block(Block{full: true, ..})
            | Self::FunctionCall(FunctionCall{full: true, ..})
            | Self::ListInitialiser(ListInitialiser{full: true, ..}) => Err(()),
            //
            //
            // recurse
            Self::Unary(Unary { arg: Some(arg), .. })
            | Self::Binary(Binary { arg_r: Some(arg), .. })
            | Self::Ternary(Ternary { failure: Some(arg), .. } | Ternary { condition: arg, .. }) => {
                arg.apply_to_last_list_initialiser(f)
            }
            //
            //
            // try recurse on non-full lists
            Self::FunctionCall(FunctionCall {
                full: false, args: vec, ..
            })
            | Self::Block(Block { elts: vec, full: false }) => vec
                .last_mut()
                .map_or(Err(()), |node| node.apply_to_last_list_initialiser(f)),
        }
    }

    /// Checks if a `{` is meant as a [`ListInitialiser`] or as a [`Block`].
    ///
    /// # Returns
    ///  - `Ok(true)` if the brace is meant as a list initaliser.
    ///  - `Ok(false)` if the brace is meant as an opening block symbol.
    ///  - `Err(op)` if the brace is illegal, because the ast is expecting a
    ///    valid leaf. `op` is the stringified version of the operator that has
    ///    an empty child. List initialiser is a valid leaf only for
    ///    [`BinaryOperator::Assign`] and [`BinaryOperator::Comma`].
    pub fn can_push_list_initialiser(&self) -> Result<bool, String> {
        match self {
            //
            //
            // can push
            Self::Binary(Binary {
                op: BinaryOperator::Assign | BinaryOperator::Comma,
                arg_r: None,
                ..
            }) => Ok(true),
            //
            Self::FunctionCall(FunctionCall {
                full: false, args: vec, ..
            })
            | Self::ListInitialiser(ListInitialiser { full: false, elts: vec })
                if vec.last().is_none_or(|node| *node == Self::Empty) =>
            {
                Ok(true)
            }
            //
            //
            // empty: can't push
            Self::Block(Block { elts, .. }) if elts.last().is_none_or(|node| *node == Self::Empty) => Ok(false),
            //
            Self::Empty
            // full: can't push
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::Block(Block { full: true, .. })
            | Self::ListInitialiser(ListInitialiser { full: true, .. })
            | Self::FunctionCall(FunctionCall { full: true, .. }) => Ok(false),
            //
            //
            // illegal leaf: can't push
            Self::Unary(Unary { op, arg: None }) => Err(op.to_string()),
            Self::Binary(Binary { op, arg_r: None, .. }) => Err(op.to_string()),
            //
            //
            // recurse
            Self::Unary(Unary { arg: Some(arg), .. })
            | Self::Binary(Binary { arg_r: Some(arg), .. })
            | Self::Ternary(Ternary { failure: Some(arg), .. } | Ternary { success: arg, .. }) => {
                arg.can_push_list_initialiser()
            }
            //
            //
            // lists
            Self::Block(Block { elts: vec, full: false })
            | Self::FunctionCall(FunctionCall { args: vec, full: false, .. })
            | Self::ListInitialiser(ListInitialiser { elts: vec, full: false }) => {
                vec.last().map_or( Ok(false), Self::can_push_list_initialiser)
            }
        }
    }

    /// Adds the colon of a [`super::TernaryOperator`].
    ///
    /// This method finds a ternary operator, and changes its reading state to
    /// failure.
    pub fn handle_colon(&mut self) -> Result<(), String> {
        match self {
            //
            //
            // success
            Self::Ternary(Ternary {
                failure: failure @ None,
                ..
            }) => {
                *failure = Some(Box::from(Self::Empty));
                Ok(())
            }
            //
            //
            // failure
            // atomic
            Self::Empty | Self::Leaf(_) | Self::ParensBlock(_)
            // non-full
            | Self::Unary(Unary { arg: None, .. })
            | Self::Binary(Binary { arg_r: None, .. })
            // full
            | Self::FunctionCall(FunctionCall { full: true, .. })
            | Self::ListInitialiser(ListInitialiser{full: true, ..})
            | Self::Block(Block{full: true, ..}) => Err("Found non-full-tree without '?' symbol.".to_owned()),
            //
            //
            // recurse
            // operators
            Self::Binary(Binary { arg_r: Some(node), .. })
            | Self::Unary(Unary { arg: Some(node), .. })
            | Self::Ternary(Ternary {
                failure: Some(node), ..
            }) => node.handle_colon(),
            // lists
            Self::FunctionCall(FunctionCall {
                full: false, args: vec, ..
            })
            | Self::ListInitialiser(ListInitialiser { full: false, elts: vec })
            | Self::Block(Block { elts: vec, full: false }) => {
                vec.last_mut().expect("Created with one elt").handle_colon()
            }
        }
    }

    /// Checks if a [`Ast`] is pushable
    ///
    /// # Returns
    ///  - `false` if one child on the right branch is empty
    ///  - `true` otherwise
    fn is_full(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Leaf(_) | Self::ParensBlock(_) => true,
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(Ternary { failure: arg, .. }) => {
                arg.as_ref().is_some_and(|node| node.is_full())
            }
            Self::Block(Block { elts: vec, full })
            | Self::ListInitialiser(ListInitialiser { full, elts: vec })
            | Self::FunctionCall(FunctionCall {
                full, args: vec, ..
            }) => *full || vec.last().is_some_and(Self::is_full),
        }
    }

    /// Pushes a node at the bottom of the [`Ast`].
    ///
    /// This methods consideres `node` as a leaf, and pushes it as a leaf into
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
            Self::Unary(Unary {
                arg: arg @ None, ..
            })
            | Self::Binary(Binary {
                arg_r: arg @ None, ..
            }) => {
                *arg = Some(Box::from(node));
                Ok(())
            }
            //
            //
            // full: ok, but create a new block
            Self::Block(Block { full: true, .. }) => {
                *self = Self::Block(Block {
                    elts: vec![mem::take(self), node],
                    full: false,
                });
                Ok(())
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
            Self::Unary(Unary { arg: Some(arg), .. })
            | Self::Binary(Binary {
                arg_r: Some(arg), ..
            })
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
            | Self::Block(Block {
                elts: vec,
                full: false,
            }) => {
                if let Some(last) = vec.last_mut()
                    && !last.is_full()
                {
                    last.push_block_as_leaf(node)
                } else {
                    vec.push(node);
                    Ok(())
                }
            }
        }
    }

    /// Tries to push an operator in the [`Ast`]
    ///
    /// This method finds, with the associatvities, precedences and arities,
    /// were to push the `op` into the [`Ast`].
    pub fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + IsComma,
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
            Self::Block(Block { full: true, .. }) => {
                *self = Self::Block(Block {
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
            | Self::Block(Block {
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
                    Ordering::Greater => {
                        if let Some(node) = arg {
                            node.push_op(op)
                        } else {
                            *arg = Some(Box::from(op.try_to_node()?));
                            Ok(())
                        }
                    }
                    Ordering::Equal => {
                        // doing whatever works ? no ! e.g.: !g(!x) gives !!g(x)
                        // for `op.try_push_op_as_root(self)`
                        if let Some(node) = arg {
                            node.push_op(op)
                        } else {
                            *arg = Some(Box::from(op.try_to_node()?));
                            Ok(())
                        }
                    }
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
                        if let Some(node) = arg {
                            node.push_op(op)
                        } else {
                            *arg = Some(Box::from(op.try_to_node()?));
                            Ok(())
                        }
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
            // explicit deragoratory rule on success block of a ternary operator
            Self::Ternary(Ternary { success: arg, .. }) => arg.push_op(op),
        }
    }

    /// Tries to conclude the arguments of a [`FunctionCall`].
    ///
    /// This method is called when `)`. It tries to make the [`FunctionCall`]
    /// the nearest to the leaves a full [`FunctionCall`].
    ///
    /// # Returns
    ///  - `true` if the function was
    pub fn try_close_function(&mut self) -> bool {
        match self {
            //
            //
            // success
            Self::FunctionCall(FunctionCall { full: full @ false, args, .. }) => {
                if !args.last_mut().is_some_and(Self::try_close_function) {
                    *full = true; }
                    true

                }
            //
            //
            // failure
            // not full
            Self::Empty
            | Self::Unary(Unary { arg: None, .. })
            | Self::Binary(Binary { arg_r: None, .. })
            | Self::Ternary(Ternary { failure: None, .. })
            // full but not function call
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::FunctionCall(FunctionCall { full: true, .. })
            | Self::Block(Block { full: true, .. })
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) => false,
            //
            //
            // recurse
            // operators
            Self::Unary(Unary { arg: Some(arg), .. })
            | Self::Binary(Binary {
                arg_r: Some(arg), ..
            })
            | Self::Ternary(Ternary {
                failure: Some(arg), ..
            }) => arg.try_close_function(),
            // list
            Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::Block(Block{elts: vec, ..}) => vec.last_mut().is_some_and(Self::try_close_function),
        }
    }

    /// Tries to create a function from the last [`Literal::Variable`].
    ///
    /// # Returns
    ///  - `true` if the function was created
    ///  - `false` if the node wasn't full, and the creation failed.
    pub fn try_make_function(&mut self) -> bool {
        match self {
            //
            //
            // success
            Self::Leaf(Literal::Variable(Variable{ name, attrs })) => {
                *self = Self::FunctionCall(FunctionCall { name: mem::replace(name, VariableName::from("")), return_attrs: mem::take(attrs), op: FunctionOperator, args: vec![], full: false }); true
            }
            //
            //
            // failure
            // not full
            Self::Empty
            | Self::Unary(Unary { arg: None, .. })
            | Self::Binary(Binary { arg_r: None, .. })
            | Self::Ternary(Ternary { failure: None, .. })
            // full but not variable
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::FunctionCall(FunctionCall { full: true, .. })
            | Self::Block(Block { full: true, .. })
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) => false,
            //
            //
            //
            // recurse
            // operators
            Self::Unary(Unary { arg: Some(arg), .. })
            | Self::Binary(Binary {
                arg_r: Some(arg), ..
            })
            | Self::Ternary(Ternary {
                failure: Some(arg), ..
            }) => arg.try_make_function(),
            // lists
            Self::FunctionCall(FunctionCall { args: vec, .. })
            | Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::Block(Block{elts: vec, ..}) => vec.last_mut().is_some_and(Self::try_make_function),
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
            Self::Block(block) => block.fmt(f),
            Self::ListInitialiser(list_initialiser) => list_initialiser.fmt(f),
            Self::ParensBlock(node) => write!(f, "({node})"),
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
