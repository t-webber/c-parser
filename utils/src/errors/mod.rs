//! Module to deal with compiler errors
//!
//! This module provides the tools to store the information on errors during
//! compile-time and display these errors to the user at the end of the
//! compilation process.

pub mod compile;
#[cfg(feature = "debug")]
pub mod debug;
pub mod display;
pub mod location;
pub mod result;
