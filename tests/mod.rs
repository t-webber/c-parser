//! Different unit tests to test the whole compilation chain.
//!
//! It is scoped as such to allow sharing code between test files.

#![expect(clippy::tests_outside_test_module, reason = "this is a test module")]

/// Tests for the parsing logic.
mod parser;
