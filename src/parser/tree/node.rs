use core::cmp::Ordering;
use core::fmt;

use super::binary::{Binary, BinaryOperator};
use super::conversions::OperatorConversions;
use super::unary::Unary;
use super::{
    repr_vec_node, Associativity, FunctionCall, ListInitialiser, Literal, Operator as _, Ternary
};

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

const SUCC_LITS_ERR: &str = "Found 2 successive literals without logical relation: ";

impl Node {
    pub fn push_block_as_leaf(&mut self, node: Self) -> Result<(), String> {
        match self {
            Self::Empty => {
                *self = node;
                Ok(())
            }

            Self::ParensBlock(old) => Err(format!("{SUCC_LITS_ERR}{old} {node}.")),
            Self::Leaf(literal) => Err(format!("{SUCC_LITS_ERR}{literal} {node}.")),
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
            Self::FunctionCall(FunctionCall { full: true, .. })
            | Self::ListInitialiser(ListInitialiser { full: true, .. }) => {
                Err(format!("{SUCC_LITS_ERR}{self}"))
            }
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

    pub fn contains_operators_on_right(&self) -> Option<String> {
        #[allow(clippy::match_same_arms)]
        match self {
            // atomic & full
            Self::Empty
            | Self::Leaf(_)
            | Self::ParensBlock(_)
            | Self::ListInitialiser(ListInitialiser { full: true, .. })
            | Self::FunctionCall(FunctionCall { full: true, .. }) => None,
            // operators
            Self::Unary(Unary { op, .. }) => Some(op.to_string()),
            Self::Ternary(_) => Some(String::from("?:")),
            Self::Binary(Binary {
                op: BinaryOperator::Assign | BinaryOperator::Comma,
                ..
            }) => None,
            Self::Binary(Binary { op, .. }) => Some(op.to_string()),
            // lists
            Self::Block(vec)
            | Self::ListInitialiser(ListInitialiser { elts: vec, .. })
            | Self::FunctionCall(FunctionCall { args: vec, .. }) => {
                vec.last().and_then(Self::contains_operators_on_right)
            }
        }
    }

    pub fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display,
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
