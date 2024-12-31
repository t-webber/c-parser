#![expect(clippy::arbitrary_source_item_ordering)]
#![allow(
    clippy::todo,
    unused_variables,
    clippy::needless_pass_by_ref_mut,
    clippy::needless_pass_by_value,
    reason = "not yet implemented"
)]

use super::Ast;
use super::types::PushInNode;
use crate::parser::tree::blocks::Block;

pub trait ControlFlow {
    fn is_full(&self) -> bool;
    fn last(&self) -> Option<&Ast>;
    fn last_mut(&mut self) -> Option<&mut Ast>;
}

macro_rules! define_control_flows {
    ($($pascal:tt,)*) => {
        #[derive(Debug, PartialEq, Eq)]
        pub enum ControlFlowKeyword {
            $($pascal,)*
        }

        #[derive(Debug, PartialEq)]
        pub enum ControlFlowNode {
            $($pascal($pascal),)*
        }


        impl ControlFlow for ControlFlowNode {
            fn is_full(&self) -> bool {
                match self {
                    $(Self::$pascal(val) => val.is_full(),)*
                }
            }

            fn last(&self) -> Option<&Ast> {
                match self {
                    $(Self::$pascal(val) => val.last(),)*
                }
            }

            fn last_mut(&mut self) -> Option<&mut Ast> {
                match self {
                    $(Self::$pascal(val) => val.last_mut(),)*
                }
            }
        }

        $(
            #[derive(Debug, PartialEq)]
            pub struct $pascal {
                full: bool,
                last: Box<Ast>//TODO: this is fake
            }

            impl ControlFlow for $pascal {
                fn is_full(&self) -> bool {
                    self.full
                }

                fn last(&self) -> Option<&Ast> {
                    if self.full {
                        None
                    } else {
                        Some(&self.last)
                    }
                }

                fn last_mut(&mut self) -> Option<&mut Ast> {
                    if self.full {
                        None
                    } else {
                        Some(&mut self.last)
                    }
                }


            }
        )*
    };
}

define_control_flows! {
    // cases & loops
    Break,
    Case,
    Continue,
    Default,
    Do,
    For,
    Goto,
    Switch,
    While,
    // condition
    Else,
    If,
    // user defined types
    Typedef,
    Struct,
    Union,
    Enum,
    //
    Return,
}

impl ControlFlowNode {
    pub fn push_colon(&mut self) -> Result<(), String> {
        todo!()
    }
    pub fn push_block_as_leaf(&mut self) -> Result<(), String> {
        todo!()
    }
    pub fn push_op<T>(&mut self, op: T) -> Result<(), String> {
        todo!()
    }
}

pub fn is_in_case_context(node: &Ast) -> bool {
    match node {
            //
            //
            // true
            Ast::ControlFlow(ControlFlowNode::Case(case)) if !case.is_full() => true,
            //
            //
            // false
            // empty
            Ast::Empty
            | Ast::Leaf(_)
            | Ast::ParensBlock(_)
            // control flows are not expressions
            | Ast::Unary(_)
            | Ast::Binary(_)
            | Ast::Ternary(_)
            | Ast::FunctionCall(_)
            | Ast::ListInitialiser(_)
            // content is full
            | Ast::Block(Block { full: true, .. }) => false,
            //
            //
            // recurse
            Ast::Block(Block { elts, full: false }) => elts.last().is_some_and(is_in_case_context),
            Ast::ControlFlow(ctrl ) => ctrl.last().is_some_and(is_in_case_context),
        }
}

impl PushInNode for ControlFlowKeyword {
    fn push_in_node(self, node: &mut Ast) -> Result<(), String> {
        todo!()
    }
}
