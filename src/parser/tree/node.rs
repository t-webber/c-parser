use core::cmp::Ordering;
use core::{fmt, mem};

use super::binary::{Binary, BinaryOperator};
use super::blocks::Block;
use super::conversions::OperatorConversions;
use super::literal::{Attribute, Literal, Variable, VariableName};
use super::traits::{Associativity, Operator as _};
use super::unary::{Unary, UnaryOperator};
use super::{FunctionCall, FunctionOperator, ListInitialiser, Ternary};
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
    pub fn add_attribute_to_left_variable(
        &mut self,
        previous_attrs: Vec<Attribute>,
    ) -> Result<(), String> {
        let make_error = |msg: &str| Err(format!("LHS: {msg} are illegal in type declarations."));

        match self {
            Self::Empty => Err("LHS: Missing identifier.".to_owned()),
            Self::Leaf(Literal::Variable(Variable { attrs, .. })) => {
                let old_attrs = mem::take(attrs);
                attrs.reserve(previous_attrs.len() + attrs.len());
                attrs.extend(previous_attrs);
                attrs.extend(old_attrs);
                Ok(())
            }
            Self::Leaf(_) => make_error("Constants"),
            Self::Unary(Unary { arg, .. }) | Self::Binary(Binary { arg_l: arg, .. }) => {
                arg.add_attribute_to_left_variable(previous_attrs)
            }
            Self::Ternary(_) => make_error("Ternary operators"),
            Self::FunctionCall(_) => make_error("Functions"),
            Self::ListInitialiser(_) => make_error("List initialisers"),
            Self::Block(_) => make_error("Blocks"),
            Self::ParensBlock(_) => make_error("Parenthesis"),
        }
    }

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
            // full lists
            | Self::Block(Block{full: true, ..})
            | Self::FunctionCall(FunctionCall{full: true, ..})
            | Self::ListInitialiser(ListInitialiser{full: true, ..}) => Err(()),
            //
            //
            // recurse
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
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
            Self::Block(Block { elts: vec, full })
            | Self::ListInitialiser(ListInitialiser { full, elts: vec })
            | Self::FunctionCall(FunctionCall {
                full, args: vec, ..
            }) => {
                !*full
                    && vec
                        .last()
                        .is_none_or(|child| child.can_push_leaf(is_user_variable))
            }
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
                arg_r,
                ..
            }) if **arg_r == Self::Empty => Ok(true),
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
            Self::Unary(Unary { op, arg }) if **arg == Self::Empty => Err(op.to_string()),
            Self::Binary(Binary { op, arg_r, .. }) if **arg_r == Self::Empty => Err(op.to_string()),
            //
            //
            // recurse
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg,  .. })
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

    /// Adds the colon of a [`TernaryOperator`](super::TernaryOperator).
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
            Self::Empty
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::FunctionCall(FunctionCall { full: true, .. })
            | Self::ListInitialiser(ListInitialiser { full: true, .. })
            | Self::Block(Block { full: true, .. }) => {
                Err("Ternary symbol mismatched: found a ':' symbol without '?'.".to_owned())
            }
            //
            //
            // recurse
            // operators
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(Ternary {
                failure: Some(arg), ..
            }) => arg.handle_colon(),
            // lists
            Self::FunctionCall(FunctionCall {
                full: false,
                args: vec,
                ..
            })
            | Self::ListInitialiser(ListInitialiser {
                full: false,
                elts: vec,
            })
            | Self::Block(Block {
                elts: vec,
                full: false,
            }) => vec.last_mut().expect("Created with one elt").handle_colon(),
        }
    }

    /// Make an [`Ast`] a LHS node
    ///
    /// This is called when an assign [`Operator`](super::Operator) is created
    /// or a function is created, to convert `*` to a type attribute. It
    /// also check that the [`Ast`] is a valid LHS.
    pub fn make_lhs(&mut self) -> Result<(), String> {
        self.make_lhs_aux(false)
    }

    fn make_lhs_aux(&mut self, push_indirection: bool) -> Result<(), String> {
        let make_error = |val: &str| {
            Err(format!(
                "LHS: expected a declaration or a modifiable lvalue, found {val}."
            ))
        };

        match self {
            // success
            Self::Leaf(Literal::Variable(var)) => {
                if push_indirection {
                    var.push_indirection()
                } else {
                    Ok(())
                }
            }
            // can't be declaration: finished
            Self::Binary(Binary {
                op:
                    BinaryOperator::StructEnumMemberAccess
                    | BinaryOperator::StructEnumMemberPointerAccess,
                ..
            }) => Ok(()),
            // recurse
            Self::Unary(Unary {
                op: UnaryOperator::Indirection,
                ..
            }) => make_error("'*' with an identifier. Change attributes ordering or remove '*'"),
            Self::Binary(Binary {
                op: BinaryOperator::Multiply,
                arg_l,
                arg_r,
            }) => {
                arg_l.make_lhs()?;
                if let Self::Leaf(Literal::Variable(old_var)) = *mem::take(arg_l) {
                    let mut new_var = old_var;
                    new_var.push_indirection()?;
                    arg_r.add_attribute_to_left_variable(new_var.attrs)?;
                    *self = mem::take(arg_r);
                    Ok(())
                } else {
                    make_error("both")
                }
            }
            Self::Binary(Binary {
                op: BinaryOperator::ArraySubscript,
                arg_l,
                ..
            }) => arg_l.make_lhs_aux(push_indirection),
            // failure
            Self::Empty => make_error("nothing"),
            Self::ParensBlock(_) => make_error("parenthesis"),
            Self::Leaf(lit) => make_error(&format!("constant literal {lit}.")),
            Self::Unary(Unary { op, .. }) => make_error(&format!("unary operator {op}")),
            Self::Binary(Binary { op, .. }) => make_error(&format!("binary operator '{op}'")),
            Self::Ternary(_) => make_error("ternary operator"),
            Self::FunctionCall(FunctionCall { full: true, .. }) => make_error("function"),
            Self::ListInitialiser(ListInitialiser { full: true, .. }) => {
                make_error("list initialiser")
            }
            Self::Block(Block { full: true, .. }) => make_error("block"),
            Self::FunctionCall(FunctionCall { .. })
            | Self::ListInitialiser(ListInitialiser { .. })
            | Self::Block(Block { .. }) => panic!("Didn't pushed assign operator low enough"),
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
            //
            //
            // full: ok, but create a new block
            // Example: {a}b
            Self::Block(Block { full: true, .. }) => {
                *self = Self::Block(Block {
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
            | Self::Block(Block {
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
                    } else if matches!(last, Self::Block(_)) {
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
        }
    }

    /// Tries to push an operator in the [`Ast`]
    ///
    /// This method finds, with the associatvities, precedences and arities,
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
            Self::FunctionCall(FunctionCall {
                full: full @ false,
                args,
                ..
            }) => {
                if !args.last_mut().is_some_and(Self::try_close_function) {
                    *full = true;
                }
                true
            }
            //
            //
            // failure
            Self::Empty
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::Block(Block { full: true, .. })
            | Self::Ternary(Ternary { failure: None, .. })
            | Self::FunctionCall(FunctionCall { full: true, .. })
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) => false,
            //
            //
            // recurse
            // operators
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(Ternary {
                failure: Some(arg), ..
            }) => arg.try_close_function(),
            // list
            Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::Block(Block { elts: vec, .. }) => {
                vec.last_mut().is_some_and(Self::try_close_function)
            }
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
            Self::Leaf(Literal::Variable(var)) => {
                *self = Self::FunctionCall(FunctionCall {
                    variable: mem::take(var),
                    op: FunctionOperator,
                    args: vec![],
                    full: false,
                });
                true
            }
            //
            //
            // failure
            Self::Empty
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::Block(Block { full: true, .. })
            | Self::Ternary(Ternary { failure: None, .. })
            | Self::FunctionCall(FunctionCall { full: true, .. })
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) => false,
            //
            //
            //
            // recurse
            // operators
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(Ternary {
                failure: Some(arg), ..
            }) => arg.try_make_function(),
            // lists
            Self::FunctionCall(FunctionCall { args: vec, .. })
            | Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::Block(Block { elts: vec, .. }) => {
                vec.last_mut().is_some_and(Self::try_make_function)
            }
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
