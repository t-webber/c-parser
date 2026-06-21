//! Different unit tests to test the whole compilation chain.
//!
//! It is scoped as such to allow sharing code between test files.

#![allow(clippy::restriction, dead_code, reason = "tests should fail")]

mod lineariser;
mod parser;
mod runner;
