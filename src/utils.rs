//! A couple of utility functions to ease the code

#![expect(clippy::arbitrary_source_item_ordering, reason = "macro usage")]

/// Macro to derive `fmt::Display` without needing to write all the boiler plate
///
/// # Example
///
/// ```ignore
/// #![feature(coverage_attribute)]
/// enum Token {
///     Symbol(char),
///     String(String),
/// }
///
/// c_parser::display!(
///     Token,
///     self,
///     f,
///     match self {
///         Self::Symbol(ch) => ch.fmt(f),
///         Self::String(value) => write!(f, "\"{value}\""),
///     }
/// );
/// ```
macro_rules! display {
    ($t:ty, $self:ident, $f:ident, $code:expr) => {
        #[coverage(off)]
        impl core::fmt::Display for $t {
            fn fmt(&$self, $f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                $code
            }
        }
    };
}

use core::fmt;

use crate::lexer::api::StringId;
use crate::parser::api::{Attribute, Literal};
use crate::{EMPTY, Res};

/// Resolves a string from a given [`StringId`].
pub struct StringResolver<T>(T, Vec<(String, StringId)>);

impl<T> StringResolver<T> {
    /// Returns the inner value held.
    pub const fn as_value(&self) -> &T {
        &self.0
    }

    /// Displays a literal.
    pub(crate) fn display_lit(&self, lit: &Literal) -> String {
        match lit {
            Literal::Char(ch) => format!("'{ch}'"),
            Literal::ConstantBool(bool) => bool.to_string(),
            Literal::Null => "null".to_owned(),
            Literal::Number(nb) => nb.to_string(),
            Literal::Str(id) => format!("\"{}\"", self.resolve(*id)),
        }
    }

    /// Displays a type expression.
    pub(super) fn display_type<A, F: Fn(&A) -> &Attribute>(&self, ty: &[A], as_attr: F) -> String {
        ty.iter()
            .map(|attr| match as_attr(attr) {
                Attribute::Indirection => "*".to_owned(),
                Attribute::Keyword(kwd) => format!("{kwd:?}"),
                Attribute::User(id) => format!("${}", self.resolve(*id)),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Returns the list of tokens and the string table.
    #[must_use]
    pub fn map_res<F: FnOnce(T) -> Res<U>, U>(self, apply: F) -> Res<StringResolver<U>> {
        apply(self.0).map(|val| StringResolver(val, self.1))
    }

    /// Returns the actual string encoded by the given id.
    pub fn resolve(&self, id: StringId) -> &str {
        self.1
            .iter()
            .find(|val| val.1 == id)
            .map(|val| &val.0)
            .expect("ids need to exist in the string table")
    }
}

impl<T> From<(T, Vec<(String, StringId)>)> for StringResolver<T> {
    fn from((value, strings): (T, Vec<(String, StringId)>)) -> Self {
        Self(value, strings)
    }
}

/// Displays the fullness, with `..` if the content is still pushable
pub const fn repr_fullness(full: bool) -> &'static str {
    if full { "" } else { ".." }
}

/// Displays an option with the [`EMPTY`] string.
#[expect(
    clippy::ref_option,
    reason = "convenient usage (always used on &Option and removes need for .as_ref)"
)]
pub fn repr_option<T: fmt::Display>(opt: &Option<T>) -> String {
    opt.as_ref().map_or_else(|| EMPTY.to_owned(), T::to_string)
}
/// Displays an option with the [`EMPTY`] string.
pub fn repr_option_vec<T: fmt::Display>(vec: &[Option<T>], sep: &str) -> String {
    vec.iter().map(repr_option).collect::<Vec<_>>().join(sep)
}

/// Displays a vector with the [`EMPTY`] string.
pub fn repr_vec<T: fmt::Display>(vec: &[T], sep: &str) -> String {
    vec.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(sep)
}

/// Struct to track if object are used or not.
pub struct SingleUse<T> {
    /// Whether the value is used or not.
    used: bool,
    /// Value held.
    value: T,
}

impl<T: Copy> SingleUse<T> {
    /// Returns the value, marking it as used.
    pub const fn as_value(&mut self) -> T {
        self.used = true;
        self.value
    }
    /// Creates a new [`SingleUse`].
    pub const fn from(value: T) -> Self {
        Self { value, used: false }
    }
    /// Returns the value only if unused.
    pub const fn try_into_value(self) -> Option<T> {
        if self.used { None } else { Some(self.value) }
    }
}

/// Safely converts u32 to usize.
#[expect(clippy::as_conversions, reason = "usize is 32 or 64")]
pub const fn u32_to_usize(val: u32) -> usize {
    val as usize
}

/// Safely converts usize to u32.
///
/// # Panics
///
/// on overflow.
pub fn usize_to_u32(val: usize) -> u32 {
    u32::try_from(val).expect("File too big, please refactor or split in multiple files.")
}

pub(crate) use display;
