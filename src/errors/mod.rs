//! Module to deal with compiler errors
//!
//! This module provides the tools to store the information on errors during
//! compile-time and display these errors to the user at the end of the
//! compilation process.

pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use, reason = "expose simple API")]

    pub use super::compile::CompileError;
    #[cfg(feature = "debug")]
    pub use super::debug::Print;
    pub use super::location::{ErrorLocation, ExtendErrorBlock, IntoError, LocationPointer};
    pub use super::result::{CompileRes, Res, SingleRes};
}

mod compile;
#[cfg(feature = "debug")]
mod debug;
mod display;
mod location;
mod result;
