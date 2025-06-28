//!Implement the user-defined-types control flow

use core::fmt;

use crate::parser::display::repr_option;
use crate::parser::keyword::attributes::UserDefinedTypes;
use crate::parser::keyword::control_flow::node::ControlFlowNode;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::push::Push;
use crate::parser::operators::api::OperatorConversions;
use crate::parser::symbols::api::BracedBlock;
use crate::parser::tree::Ast;

/// Keyword expects an identifier and a braced block: `struct Blob {}`
#[derive(Debug)]
pub struct IdentBlockCtrl {
    /// User defined type definition
    block: Option<BracedBlock>,
    /// User defined type name
    ident: Option<String>,
    /// User defined type type
    keyword: IdentBlockKeyword,
}

impl ControlFlow for IdentBlockCtrl {
    type Keyword = IdentBlockKeyword;

    fn as_ast(&self) -> Option<&Ast> {
        None
    }

    fn as_ast_mut(&mut self) -> Option<&mut Ast> {
        None
    }

    fn fill(&mut self) {}

    fn from_keyword(keyword: Self::Keyword) -> Self {
        Self { keyword, ident: None, block: None }
    }

    fn is_full(&self) -> bool {
        self.block.is_some()
    }

    fn push_colon(&mut self) -> bool {
        false
    }

    fn push_semicolon(&mut self) -> bool {
        false
    }
}

impl Push for IdentBlockCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "user-defined type");
        debug_assert!(!self.is_full(), "");
        match (&mut self.ident, &mut self.block, ast) {
            (_, Some(_), node) => panic!("Tried to push {node} on full control flow."),
            (_, None, Ast::BracedBlock(braced)) => self.block = Some(braced),
            (None, None, Ast::Variable(var)) => {
                self.ident = Some(var.into_user_defined_name()?);
            }
            (Some(_), None, Ast::Variable(_)) => {
                return Err(
                    "Found 2 successive variable: expected block after variable.".to_owned()
                );
            }
            (
                _,
                _,
                node @ (Ast::Empty
                | Ast::Leaf(_)
                | Ast::Cast(_)
                | Ast::Unary(_)
                | Ast::Binary(_)
                | Ast::Ternary(_)
                | Ast::ParensBlock(_)
                | Ast::ControlFlow(_)
                | Ast::FunctionCall(_)
                | Ast::ListInitialiser(_)
                | Ast::FunctionArgsBuild(_)),
            ) => {
                return Err(format!(
                    "Tried to push invalid leaf to struct definition. Expected block or name, found {node}"
                ));
            }
        }
        Ok(())
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "user-defined type");
        debug_assert!(!self.is_full(), "");
        if let Some(BracedBlock { elts, full: false }) = &mut self.block {
            if let Some(last) = elts.last_mut() {
                last.push_op(op)
            } else {
                elts.push(op.try_to_node()?);
                Ok(())
            }
        } else if self.block.is_some() {
            Err("Failed to push operator to full control flow".to_owned())
        } else {
            Err("Failed to push operator: missing identifier".to_owned())
        }
    }
}

#[expect(clippy::min_ident_chars, reason = "don't rename trait's method params")]
#[coverage(off)]
impl fmt::Display for IdentBlockCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<{} {} {}>",
            match self.keyword {
                IdentBlockKeyword::Enum => "enum",
                IdentBlockKeyword::Struct => "struct",
                IdentBlockKeyword::Union => "union",
            },
            repr_option(&self.ident),
            repr_option(&self.block)
        )
    }
}

/// C control flow keywords that have the [`IdentBlockCtrl`] structure.
#[derive(Debug, PartialEq, Eq)]
pub enum IdentBlockKeyword {
    /// `enum A { }`
    Enum,
    /// `struct A { }`
    Struct,
    /// `union A { }`
    Union,
}

impl UserDefinedTypes {
    /// Tries to convert an attribute keyword to a control flow
    ///
    /// `struct`, `enum` and `union` can be both attribute (whilst declaring a
    /// variable) and control flow (whilst defining a type). By default, when
    /// the `typedef` word wasn't found, these keywords are interpreted as
    /// attributes. If we find out they were in fact control flow nodes, we use
    /// this function to convert them.
    pub const fn to_control_flow(
        &self,
        ident: Option<String>,
        block: Option<BracedBlock>,
    ) -> ControlFlowNode {
        let keyword = match self {
            Self::Struct => IdentBlockKeyword::Struct,
            Self::Union => IdentBlockKeyword::Union,
            Self::Enum => IdentBlockKeyword::Enum,
        };
        ControlFlowNode::IdentBlock(IdentBlockCtrl { block, ident, keyword })
    }
}
