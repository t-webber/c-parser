//! Implementation of the `typedef` keyword.

use core::fmt;

use super::node::ControlFlowNode;
use crate::EMPTY;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::repr_option;
use crate::parser::types::Ast;
use crate::parser::types::variable::Variable;

/// Control flow for `typedef` keyword.
#[derive(Debug, PartialEq, Default)]
pub enum Typedef {
    /// Typedef in a type definition
    ///
    /// # Examples
    ///
    /// ```c
    /// typedef struct {} name;
    /// ```
    Definition(Box<ControlFlowNode>, Option<String>),
    /// Typedef without any arguments
    #[default]
    None,
    /// Typedef in a type redefinition
    ///
    /// # Examples
    ///
    /// ```c
    /// typedef struct A name;
    /// typedef int name2;
    /// ```
    Type(Variable),
}

impl Typedef {
    /// Checks if the [`Typedef`] is pushable
    pub const fn is_full(&self) -> bool {
        match self {
            Self::Definition(_, name) => name.is_some(),
            Self::Type(var) => var.is_full(),
            Self::None => false,
        }
    }
}

impl Push for Typedef {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        println!("\tPushing ast {ast} in ctrl {self}");
        if let Ast::Variable(mut new_var) = ast {
            match self {
                Self::Definition(_, Some(_)) => panic!("typedef full"),
                Self::Definition(ctrl, current_name) => {
                    if let Some(child) = ctrl.get_ast_mut() {
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
                Self::Type(current_var) => current_var.extend(new_var)?,
                Self::None => *self = Self::Type(new_var),
            }
            Ok(())
        } else if let Ast::ControlFlow(new_ctrl) = ast {
            match self {
                Self::Definition(_, Some(_)) => panic!("typedef full"),
                Self::Definition(ctrl, None) => ctrl.push_block_as_leaf(Ast::ControlFlow(new_ctrl)),
                Self::None => {
                    *self = Self::Definition(Box::new(new_ctrl), None);
                    Ok(())
                }
                Self::Type(_) => Err("Found control flow after typedef name.".to_owned()),
            }
        } else {
            match self {
                Self::Definition(ctrl, None) => ctrl.push_block_as_leaf(ast),
                Self::Type(var) => {
                    if let Some((user_type, name)) = var.get_partial_typedef() {
                        let mut ctrl = user_type.to_control_flow(name, None);
                        ctrl.push_block_as_leaf(ast)?;
                        *self = Self::Definition(Box::new(ctrl), None);
                        Ok(())
                    } else {
                        Err(format!(
                            "Tried to push illegal ast {ast} in typedef {self}."
                        ))
                    }
                }
                Self::Definition(_, Some(_)) | Self::None => Err(format!(
                    "Tried to push illegal ast {ast} in typedef {self}."
                )),
            }
        }
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: OperatorConversions + fmt::Display + Copy,
    {
        #[cfg(feature = "debug")]
        println!("\tPushing op {op} in typedef {self}");
        match self {
            Self::Definition(_, Some(_)) => panic!("Pushed in full"),
            Self::Definition(ctrl, None) => ctrl.push_op(op),
            Self::None => Err(format!("Illegal symbol {op} for typedef.")),
            Self::Type(var) => var.push_op(op),
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Typedef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Definition(node, name) => write!(f, "<typedef {node} {}>", repr_option(name)),
            Self::Type(variable) => write!(f, "<typedef {variable}>"),
            Self::None => write!(f, "<typedef {EMPTY}>"),
        }
    }
}
