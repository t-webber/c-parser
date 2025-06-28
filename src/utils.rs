//! A couple of utility functions to ease the code

#![expect(clippy::arbitrary_source_item_ordering, reason = "macro usage")]

/// Macro to derive `fmt::Display` without needing to write all the boiler plate
///
/// # Example
///
/// ```
///
macro_rules! display {
    ($t:ty, $self:ident, $f:ident, $code:expr) => {
        #[coverage(off)]
        #[expect(clippy::min_ident_chars, reason = "don't rename trait's method params")]
        impl core::fmt::Display for $t {
            fn fmt(&$self, $f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                $code
            }
        }

    };
}

pub(crate) use display;
