//! Module to parse a list of tokens into an Abstract Syntax Tree.
//!
//! This module doesn't check that the tree is valid, and only handles trivial
//! errors detection while building the AST.

#[expect(clippy::inline_modules, reason = "clearer api")]
pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use, reason = "expose simple API")]

    pub use super::literal::{Attribute, Literal};
    pub use super::parse_content::parse;
    pub use super::symbols::api::FunctionCall;
    pub use super::tree::Ast;
    pub use super::variable::api::{
        AttributeVariable, Declaration, DeclarationValue, Variable, VariableValue
    };
}

mod display;
mod keyword;
mod literal;
mod modifiers;
mod operators;
mod parse_content;
mod state;
mod symbols;
mod tree;
mod variable;
