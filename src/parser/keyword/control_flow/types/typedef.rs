//! Implementation of the `typedef` keyword.

use core::fmt;

use crate::EMPTY;
use crate::parser::keyword::control_flow::keyword::ControlFlowKeyword;
use crate::parser::keyword::control_flow::node::ControlFlowNode;
use crate::parser::keyword::control_flow::traits::ControlFlow;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::types::Ast;
use crate::parser::types::variable::Variable;
use crate::parser::{repr_fullness, repr_option};

/// Control flow for `typedef` keyword.
#[derive(Debug, PartialEq, Default)]
pub enum TypedefCtrl {
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

impl ControlFlow for TypedefCtrl {
    type Keyword = ();

    fn fill(&mut self) {}

    fn from_keyword((): Self::Keyword) -> Self {
        Self::default()
    }

    fn get_ast(&self) -> Option<&Ast> {
        match self {
            Self::Definition(ctrl, None) => ctrl.get_ast(),
            Self::None | Self::Type(_) | Self::Definition(_, Some(_)) => None,
        }
    }

    fn get_keyword(&self) -> ControlFlowKeyword {
        ControlFlowKeyword::Typedef
    }

    fn get_mut(&mut self) -> Option<&mut Ast> {
        match self {
            Self::Definition(ctrl, None) => ctrl.get_mut(),
            Self::None | Self::Type(_) | Self::Definition(_, Some(_)) => None,
        }
    }

    fn is_full(&self) -> bool {
        match self {
            Self::Definition(_, ident) => ident.is_some(),
            Self::None => false,
            Self::Type(var) => var.is_full(),
        }
    }

    fn push_colon(&mut self) -> bool {
        match self {
            Self::Definition(ctrl, None) => ctrl.push_colon(),
            Self::Definition(_, Some(_)) | Self::None | Self::Type(_) => false,
        }
    }

    fn push_semicolon(&mut self) -> bool {
        match self {
            Self::Definition(ctrl, None) => ctrl.push_semicolon(),
            Self::Definition(_, Some(_)) | Self::None | Self::Type(_) => false,
        }
    }
}

impl Push for TypedefCtrl {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        println!("\tPushing {ast} in typedef {self}");
        debug_assert!(!self.is_full(), "");
        if let Ast::Variable(mut new_var) = ast {
            match self {
                Self::Definition(_, Some(_)) => panic!("typedef full"),
                Self::Definition(ctrl, current_name @ None) => {
                    if let Some(child) = ctrl.get_mut() {
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
        println!("\tPushing {op} in typedef {self}");
        debug_assert!(!self.is_full(), "");
        match self {
            Self::Definition(_, Some(_)) => panic!("Pushed in full"),
            Self::Definition(ctrl, None) => ctrl.push_op(op),
            Self::None => Err(format!("Illegal symbol {op} for typedef.")),
            Self::Type(var) => var.push_op(op),
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for TypedefCtrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<typedef {}{}>",
            match self {
                Self::Definition(node, name) => format!("{node} {}", repr_option(name)),
                Self::Type(variable) => variable.to_string(),
                Self::None => EMPTY.to_owned(),
            },
            repr_fullness(self.is_full())
        )
    }
}
