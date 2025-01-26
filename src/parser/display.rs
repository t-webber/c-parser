//! Defines functions to display complex types, like options and vectors.

use core::fmt;

use crate::EMPTY;

/// Displays the fullness, with `..` if the content is still pushable
pub const fn repr_fullness(full: bool) -> &'static str {
    if full { "" } else { ".." }
}

/// Displays an option with the [`EMPTY`] string.
#[expect(clippy::ref_option)]
pub fn repr_option<T: fmt::Display>(opt: &Option<T>) -> String {
    opt.as_ref().map_or_else(|| EMPTY.to_owned(), T::to_string)
}
/// Displays an option with the [`EMPTY`] string.
pub fn repr_option_vec<T: fmt::Display>(vec: &[Option<T>]) -> String {
    vec.iter().map(repr_option).collect::<Vec<_>>().join(", ")
}

/// Displays a vector with the [`EMPTY`] string.
pub fn repr_vec<T: fmt::Display>(vec: &[T]) -> String {
    vec.iter()
        .map(|node| format!("{node}"))
        .collect::<Vec<_>>()
        .join(", ")
}
