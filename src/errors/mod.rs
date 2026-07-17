//! Module to deal with compiler errors
//!
//! This module provides the tools to store the information on errors during
//! compile-time and display these errors to the user at the end of the
//! compilation process.

#![expect(clippy::inline_modules, reason = "clearer api")]
pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use, reason = "expose simple API")]

    pub use super::compile::CompileError;
    #[cfg(feature = "debug")]
    pub use super::debug::Print;
    pub use super::error_location::ErrorLocation;
    pub use super::located::Located;
    pub use super::location_ptr::LocationPointer;
    pub use super::result::{CompileRes, Res};
}

mod compile;
#[cfg(feature = "debug")]
mod debug;
mod display;
mod error_location;
mod located;
mod location_ptr;
mod result;
