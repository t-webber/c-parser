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
