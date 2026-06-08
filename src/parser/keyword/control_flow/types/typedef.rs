//! Implementation of the `typedef` keyword.

use core::fmt;
use core::mem::take;

use crate::EMPTY;
use crate::errors::api::ErrorLocation;
use crate::parser::display::{repr_fullness, repr_option};
use crate::parser::keyword::control_flow::node::ControlFlowNode;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::push::Push;
use crate::parser::operators::api::OperatorConversions;
use crate::parser::tree::Ast;
use crate::parser::variable::Variable;
use crate::parser::variable::api::VariableConversion as _;
use crate::utils::display;

/// Control flow for `typedef` keyword.
#[derive(Debug)]
pub enum TypedefCtrl {
    /// Typedef in a type definition
    ///
    /// # Examples
    ///
    /// ```c
    /// typedef struct {} name;
    /// ```
    Definition(ErrorLocation, Box<ControlFlowNode>, Option<String>),
    /// Typedef without any arguments
    None(ErrorLocation),
    /// Typedef in a type redefinition
    ///
    /// # Examples
    ///
    /// ```c
    /// typedef struct A name;
    /// typedef int name2;
    /// ```
    Type(ErrorLocation, Variable),
}

impl ControlFlow for TypedefCtrl {
    type Keyword = ();

    fn as_ast(&self) -> Option<&Ast> {
        match self {
            Self::Definition(_, ctrl, None) => ctrl.as_ast(),
            Self::None(..) | Self::Type(..) | Self::Definition(_, _, Some(..)) => None,
        }
    }

    fn as_ast_mut(&mut self) -> Option<&mut Ast> {
        match self {
            Self::Definition(_, ctrl, None) => ctrl.as_ast_mut(),
            Self::None(..) | Self::Type(..) | Self::Definition(_, _, Some(..)) => None,
        }
    }

    fn fill(&mut self) {}

    fn from_keyword((): Self::Keyword, keyword_location: ErrorLocation) -> Self {
        Self::None(keyword_location)
    }

    fn is_full(&self) -> bool {
        match self {
            Self::Definition(_, _, ident) => ident.is_some(),
            Self::None(_) => false,
            Self::Type(_, var) => var.is_full(),
        }
    }

    fn push_colon(&mut self) -> bool {
        match self {
            Self::Definition(_, ctrl, None) => ctrl.push_colon(),
            Self::Definition(_, _, Some(..)) | Self::None(_) | Self::Type(..) => false,
        }
    }

    fn push_semicolon(&mut self) -> bool {
        match self {
            Self::Definition(_, ctrl, None) => ctrl.push_semicolon(),
            Self::Definition(_, _, Some(..)) | Self::None(_) | Self::Type(..) => false,
        }
    }
}

impl Push for TypedefCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_leaf(&ast, self, "typedef");
        debug_assert!(!self.is_full(), "");
        if let Ast::Variable(mut new_var) = ast {
            match self {
                Self::Definition(_, _, Some(..)) => unreachable!("typedef full"),
                Self::Definition(_, ctrl, current_name @ None) => {
                    if let Some(child) = ctrl.as_ast_mut() {
                        child.push_block_as_leaf(Ast::Variable(new_var))?;
                    } else if current_name.is_none()
                        && let Some(new_name) = new_var.take_user_defined()
                    {
                        *current_name = Some(new_name);
                    } else {
                        return Err(format!(
                            "Tried to push variable {new_var} in partially full typedef {self}."
                        ));
                    }
                }
                Self::Type(_, current_var) => current_var.extend(new_var)?,
                Self::None(location) => *self = Self::Type(take(location), new_var),
            }
            Ok(())
        } else if let Ast::ControlFlow(new_ctrl) = ast {
            match self {
                Self::Definition(_, _, Some(..)) => unreachable!("typedef full"),
                Self::Definition(_, ctrl, None) =>
                    ctrl.push_block_as_leaf(Ast::ControlFlow(new_ctrl)),
                Self::None(location) => {
                    *self = Self::Definition(take(location), Box::new(new_ctrl), None);
                    Ok(())
                }
                Self::Type(..) => Err("Found control flow after typedef name.".to_owned()),
            }
        } else {
            match self {
                Self::Definition(_, ctrl, None) => ctrl.push_block_as_leaf(ast),
                Self::Type(location, var) =>
                    if let Some((user_type, name)) = var.as_partial_typedef() {
                        let mut ctrl = user_type.to_control_flow(name, None, ErrorLocation::None);
                        ctrl.push_block_as_leaf(ast)?;
                        *self = Self::Definition(take(location), Box::new(ctrl), None);
                        Ok(())
                    } else {
                        Err(format!("Tried to push illegal ast {ast} in typedef {self}."))
                    },
                Self::Definition(_, _, Some(..)) | Self::None(_) =>
                    Err(format!("Tried to push illegal ast {ast} in typedef {self}.")),
            }
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "typedef");
        debug_assert!(!self.is_full(), "");
        match self {
            Self::Definition(_, _, Some(..)) => unreachable!("Pushed in full"),
            Self::Definition(_, ctrl, None) => ctrl.push_op(op),
            Self::None(_) => Err(format!("Illegal symbol {op} for typedef.")),
            Self::Type(_, var) => var.push_op(op),
        }
    }
}

display!(
    TypedefCtrl,
    self,
    f,
    write!(
        f,
        "<typedef {}{}>",
        match self {
            Self::Definition(_, node, name) => format!("{node} {}", repr_option(name)),
            Self::Type(_, variable) => variable.to_string(),
            Self::None(_) => EMPTY.to_owned(),
        },
        repr_fullness(self.is_full())
    )
);
