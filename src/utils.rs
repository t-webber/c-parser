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

/// Display for a [`Vec`] using space as a delimiter.
pub fn repr_vec_space<T: ToString>(vec: &[T]) -> String {
    vec.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Display for a [`Vec`] using space as a delimiter.
pub fn repr_vec_comma_space<T: ToString>(vec: &[Vec<T>]) -> String {
    vec.iter()
        .map(|inner_vec| {
            inner_vec
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(", ")
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

pub(crate) use display;
