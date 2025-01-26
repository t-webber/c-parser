//! Module that defines the handlers and modifiers to transform a simple node
//! into a complex one. For instance, it contains functions to convert a leaf to
//! a function call or to create a list initialiser from nothing.

pub mod functions;
pub mod list_initialiser;
pub mod make_lhs;
pub mod push;
