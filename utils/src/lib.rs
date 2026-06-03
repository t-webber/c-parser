#![doc = include_str!("../../docs/README.md")]
#![feature(try_trait_v2, try_trait_v2_residual, coverage_attribute)]
#![allow(clippy::pub_use, reason = "expose simple API")]

mod display;
mod errors;

pub use errors::compile::CompileError;
#[cfg(feature = "debug")]
pub use errors::debug::Print;
pub use errors::location::{ErrorLocation, ExtendErrorBlock, IntoError, LocationPointer};
pub use errors::result::{CompileRes, Res};
