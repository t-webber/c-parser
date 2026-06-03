#![doc = include_str!("../../docs/README.md")]
#![feature(
    is_ascii_octdigit,
    f128,
    pattern,
    try_trait_v2,
    coverage_attribute,
    stmt_expr_attributes,
    macro_metavar_expr_concat,
    try_trait_v2_residual
)]
//! Module to parse a list of tokens into an Abstract Syntax Tree.
//!
//! This module doesn't check that the tree is valid, and only handles trivial
//! errors detection while building the AST.

pub use parse_content::parse_tokens;

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
