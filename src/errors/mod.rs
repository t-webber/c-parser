//! Module to deal with compiler errors
//!
//! This module provides the tools to store the information on errors during
//! compile-time and display these errors to the user at the end of the
//! compilation process.

pub mod api {
    //! Api module to choose what functions to export.

    #![allow(clippy::pub_use)]

    pub use super::compile::CompileError;
    #[cfg(feature = "debug")]
    pub use super::debug::Print;
    pub use super::location::Location;
    pub use super::result::{CompileRes, Res, SingleRes};
}

mod compile;
#[cfg(feature = "debug")]
mod debug;
mod display;
mod location;
mod result;

/// Wrapper to turn off the coverage checks on [`debug_assert!`]
///
/// This functions still is replaced by void in `--release` mod, thanks to the
/// `#[inline(always)]` flag, that tells the compiler to inline with an empty
/// body.
#[coverage(off)]
#[inline(always)]
#[expect(clippy::inline_always)]
pub fn dbg_assert(cond: bool, msg: &str) {
    debug_assert!(cond, "{msg}");
}
