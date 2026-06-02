//! Different unit tests to test the whole compilation chain.
//!
//! It is scoped as such to allow sharing code between test files.

#![expect(clippy::restriction, reason = "tests should fail")]

/// Tests for the lexing logic.
mod lexer;
/// Tests for the parsing logic.
mod parser;
