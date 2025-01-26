//! Module to parse a list of tokens into an Abstract Syntax Tree.
//!
//! This module doesn't check that the tree is valid, and only handles trivial
//! errors detection while building the AST.

pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use)]

    pub use super::parse_content::parse_tokens;
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
