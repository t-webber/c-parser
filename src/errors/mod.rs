//! Module to deal with compiler errors
//!
//! This module provides the tools to store the information on errors during
//! compile-time and display these errors to the user at the end of the
//! compilation process.

pub mod api {
    #![allow(clippy::pub_use)]

    pub use super::compile::CompileError;
    pub use super::location::Location;
    pub use super::result::Res;
}

mod compile;
mod display;
mod location;
mod result;
