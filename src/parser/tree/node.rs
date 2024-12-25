use core::cmp::Ordering;
use core::{fmt, mem};

use super::binary::{Binary, BinaryOperator};
use super::conversions::OperatorConversions;
use super::traits::{Associativity, IsComma, Operator as _};
use super::unary::Unary;
use super::{repr_vec_node, FunctionCall, FunctionOperator, ListInitialiser, Literal, Ternary};

#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Default, PartialEq)]
pub enum Node {
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
    Block(Vec<Node>),
    // parenthesis
    ParensBlock(Box<Node>),
    // TODO: while, for, goto, etc; CompoundLiteral(CompoundLiteral),; SpecialUnary(SpecialUnary),
}

impl Node {
    pub fn can_push_list_initialiser(&self) -> Result<bool, String> {
        match self {
            // can push list initialiser
            Self::Binary(Binary {
                op: BinaryOperator::Assign | BinaryOperator::Comma,
                arg_r: None,
                ..
            }) => Ok(true),
            Self::FunctionCall(FunctionCall {
                full: false,
                args: vec,
                ..
            })
            | Self::ListInitialiser(ListInitialiser {
                full: false,
                elts: vec,
            }) if vec.last().is_none_or(|node| *node == Self::Empty) => Ok(true),
            // full && can't push
            Self::Empty
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::ListInitialiser(ListInitialiser { full: true, .. })
            | Self::FunctionCall(FunctionCall { full: true, .. }) => Ok(false),
            // not full && can't push
            Self::Unary(Unary { op, arg: None }) => Err(op.to_string()),
            Self::Binary(Binary {
                op, arg_r: None, ..
            }) => Err(op.to_string()),
            Self::Ternary(Ternary {
                op, failure: None, ..
            }) => Err(op.to_string()),
            // not full & recuse
            Self::Unary(Unary { arg: Some(arg), .. })
            | Self::Binary(Binary {
                arg_r: Some(arg), ..
            })
            | Self::Ternary(Ternary {
                failure: Some(arg), ..
            }) => arg.can_push_list_initialiser(),
            // lists
            Self::Block(vec)
            | Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::FunctionCall(FunctionCall { args: vec, .. }) => vec
                .last()
                .map_or_else(|| Ok(true), Self::can_push_list_initialiser),
        }
    }

    #[allow(clippy::min_ident_chars)]
    pub fn edit_list_initialiser<T, F: Fn(&mut Vec<Self>, &mut bool) -> T>(
        &mut self,
        f: &F,
    ) -> Result<T, ()> {
        match self {
            // success
            Self::ListInitialiser(ListInitialiser {
                elts,
                full: full @ false,
            }) => {
                if let Some(last) = elts.last_mut() {
                    if let res @ Ok(_) = last.edit_list_initialiser(f) {
                        return res;
                    }
                }
                Ok(f(elts, full))
            }
            // recurse
            Self::Unary(Unary { arg: Some(arg), .. })
            | Self::Binary(Binary {
                arg_r: Some(arg), ..
            })
            | Self::Ternary(
                Ternary {
                    failure: Some(arg), ..
                }
                | Ternary { condition: arg, .. },
            ) => arg.edit_list_initialiser(f),
            // try recurse
            Self::FunctionCall(FunctionCall {
                full: false,
                args: vec,
                ..
            })
            | Self::Block(vec) => vec
                .last_mut()
                .map_or_else(|| Err(()), |node| node.edit_list_initialiser(f)),
            // failure
            Self::Empty
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::Unary(_)
            | Self::Binary(_)
            | Self::FunctionCall(_)
            | Self::ListInitialiser(_) => Err(()),
        }
    }

    pub fn handle_colon(&mut self) -> Result<(), String> {
        match self {
            Self::Ternary(Ternary {
                failure: failure @ None,
                ..
            }) => {
                *failure = Some(Box::from(Self::Empty));
                Ok(())
            }
            Self::Empty | Self::Leaf(_) | Self::ParensBlock(_) => Err(
                "Found unexpected colon. Missing '?' for ternary operator, or 'goto' keyword"
                    .into(),
            ),
            Self::Binary(Binary {
                arg_r: Some(node), ..
            })
            | Self::Unary(Unary {
                arg: Some(node), ..
            })
            | Self::Ternary(Ternary {
                failure: Some(node),
                ..
            }) => node.handle_colon(),
            Self::FunctionCall(FunctionCall {
                full: false,
                args: vec,
                ..
            })
            | Self::ListInitialiser(ListInitialiser {
                full: false,
                elts: vec,
            })
            | Self::Block(vec) => vec.last_mut().expect("Created with one elt").handle_colon(),
            Self::Unary(_) | Self::Binary(_) | Self::FunctionCall(_) | Self::ListInitialiser(_) => {
                Err("Found non-full-tree without '?' symbol.".to_owned())
            }
        }
    }

    pub fn push_block_as_leaf(&mut self, node: Self) -> Result<(), String> {
        match self {
            Self::Empty => {
                *self = node;
                Ok(())
            }

            Self::ParensBlock(old) => Err(make_successive_literal_error(
                "Parenthesis group",
                old,
                node,
            )),
            Self::Leaf(literal) => Err(make_successive_literal_error("Literal", literal, node)),
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
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary {
                arg_r: arg @ None, ..
            }) => {
                *arg = Some(Box::from(node));
                Ok(())
            }
            Self::FunctionCall(FunctionCall { full: true, .. }) => {
                Err(make_successive_literal_error("Function call", self, node))
            }

            Self::ListInitialiser(ListInitialiser { full: true, .. }) => Err(
                make_successive_literal_error("List initialiser", self, node),
            ),

            Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::FunctionCall(FunctionCall { args: vec, .. })
            | Self::Block(vec) => {
                if let Some(last) = vec.last_mut() {
                    last.push_block_as_leaf(node)
                } else {
                    *vec = vec![node];
                    Ok(())
                }
            }
        }
    }

    pub fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + IsComma,
    {
        match self {
            // self empty
            Self::Empty => {
                //TODO check for unary
                op.try_convert_and_erase_node(self)
            }

            // self is a non-modifiable block
            Self::ListInitialiser(ListInitialiser { full: true, .. })
            | Self::FunctionCall(FunctionCall { full: true, .. })
            | Self::Leaf(_)
            | Self::ParensBlock(_) => op.try_push_op_as_root(self),

            // self is operator
            Self::Unary(Unary { op: old_op, arg }) => {
                match old_op.precedence().cmp(&op.precedence()) {
                    Ordering::Less => op.try_push_op_as_root(self),
                    Ordering::Greater => {    if let Some(node) = arg {
                        node.push_op(op)
                    } else {
                        *arg = Some(Box::from(op.try_to_node()?));
                        Ok(())
                    }
                }
            , /* check unary */
                    Ordering::Equal => {
                        // doing whatever works ?
                        op.try_push_op_as_root(self)
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
            Self::Ternary(Ternary { success: arg, .. }) => arg.push_op(op),

            // self pushable and not full
            Self::FunctionCall(FunctionCall { args, .. }) if op.is_comma() => {
                args.push(Self::Empty);
                Ok(())
            }
            Self::FunctionCall(FunctionCall { args: vec, .. })
            | Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::Block(vec) => {
                if let Some(last) = vec.last_mut() {
                    last.push_op(op)
                } else {
                    //TODO: check for unary
                    op.try_convert_and_erase_node(self)
                }
            }
        }
    }

    pub fn try_close_function(&mut self) -> bool {
        match self {
            Self::FunctionCall(FunctionCall { full: full @ false, .. }) => {*full =true; true }
            // not full
            Self::Empty
            | Self::Unary(Unary { arg: None, .. })
            | Self::Binary(Binary { arg_r: None, .. })
            | Self::Ternary(Ternary { failure: None, .. })
            // full but not variable
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::FunctionCall(FunctionCall { full: true, .. })
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) => false,
            // recurse on right
            Self::Unary(Unary { arg: Some(arg), .. })
            | Self::Binary(Binary {
                arg_r: Some(arg), ..
            })
            | Self::Ternary(Ternary {
                failure: Some(arg), ..
            }) => arg.try_close_function(),
            // recurse on last
            Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::Block(vec) => vec.last_mut().is_some_and(Self::try_close_function),
        }
    }

    pub fn try_make_function(&mut self) -> bool {
        match self {
            Self::Leaf(Literal::Variable(var)) => {
                let name = mem::take(var);
                *self = Self::FunctionCall(FunctionCall { name, op: FunctionOperator, args: vec![], full: false }); true
            }
            // not full
            Self::Empty
            | Self::Unary(Unary { arg: None, .. })
            | Self::Binary(Binary { arg_r: None, .. })
            | Self::Ternary(Ternary { failure: None, .. })
            // full but not variable
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::FunctionCall(FunctionCall { full: true, .. })
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) => false,
            // recurse on right
            Self::Unary(Unary { arg: Some(arg), .. })
            | Self::Binary(Binary {
                arg_r: Some(arg), ..
            })
            | Self::Ternary(Ternary {
                failure: Some(arg), ..
            }) => arg.try_make_function(),
            // recurse on last
            Self::FunctionCall(FunctionCall { args: vec, .. })
            | Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::Block(vec) => vec.last_mut().is_some_and(Self::try_make_function),
        }
    }
}

#[allow(clippy::min_ident_chars)]
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "\u{2205} "),
            Self::Binary(val) => val.fmt(f),
            Self::FunctionCall(val) => val.fmt(f),
            Self::Leaf(val) => val.fmt(f),
            Self::Ternary(val) => val.fmt(f),
            Self::Unary(val) => val.fmt(f),
            Self::Block(vec) => write!(f, "[{}]", repr_vec_node(vec)),
            Self::ListInitialiser(list_initialiser) => list_initialiser.fmt(f),
            Self::ParensBlock(node) => write!(f, "({node})"),
        }
    }
}

fn make_successive_literal_error<T: fmt::Display, U: fmt::Display>(
    old_type: &str,
    old: T,
    new: U,
) -> String {
    format!("Found 2 consecutive literals: {old_type} {old} followed by {new}.")
}
