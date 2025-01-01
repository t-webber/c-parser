//! Module that defines the number types

#![allow(clippy::arbitrary_source_item_ordering)]

pub mod arch_types {
    //! Types sizes defined for the different architectures.
    #![allow(clippy::missing_docs_in_private_items)]

    pub type Int = i32;
    #[cfg(target_pointer_width = "32")]
    pub type Long = Int;
    #[cfg(target_pointer_width = "64")]
    pub type Long = LongLong;
    pub type LongLong = i64;
    pub type Float = f32;
    pub type Double = f64;
    pub type LongDouble = f128;
    pub type UInt = u32;
    #[cfg(target_pointer_width = "32")]
    pub type ULong = UInt;
    #[cfg(target_pointer_width = "64")]
    pub type ULong = ULongLong;
    pub type ULongLong = u64;

    pub type FloatIntPart = u32;
    pub type DoubleIntPart = u64;
    pub type LongDoubleIntPart = u128;
}

use core::fmt;

use arch_types::{Double, Float, Int, Long, LongDouble, LongLong, UInt, ULong, ULongLong};

/// Defines the [`Number`] and [`NumberType`] enums
macro_rules! define_nb_types {
    ($($t:ident)*) => {
        /// Token value for a number constant
        #[derive(Debug, PartialEq)]
        pub enum Number {
            $(
                /// $t C type
                $t($t),
            )*
        }

        pub enum NumberType {
            $($t,)*
        }

    };
}

/// String prefix used at all the beginnings of error messages.
pub const ERR_PREFIX: &str = "Invalid number constant type: ";

/// Base of a number representation.
pub enum Base {
    /// Binary representation: `[0-1]`.
    Binary,
    /// Decimal representation: `[0-10]`.
    Decimal,
    /// Hexadecimal representation: `[0-16]`.
    Hexadecimal,
    /// Octal representation: `[0-8]`.
    Octal,
}

impl Base {
    /// Returns the prefix size for this specific base
    ///
    /// | Base        | Prefix | Size |
    /// | :---------: | :----: | :--: |
    /// | Binary      | "0b"   | 2    |
    /// | Hexadecimal | "0x"   | 2    |
    /// | Decimal     | ∅     | 0    |
    /// | Octal       | "0"    | 1    |
    pub const fn prefix_size(&self) -> usize {
        match self {
            Self::Binary | Self::Hexadecimal => 2,
            Self::Decimal => 0,
            Self::Octal => 1,
        }
    }

    /// Returns a string to print the base inside errors.
    pub const fn repr(&self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Decimal => "decimal",
            Self::Hexadecimal => "hexadecimal",
            Self::Octal => "octal",
        }
    }
}

define_nb_types!(Int Long LongLong Float Double LongDouble UInt ULong ULongLong);

#[expect(
    clippy::min_ident_chars,
    clippy::match_same_arms,
    clippy::as_conversions
)]
impl fmt::Display for Number {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Int(x) => x.to_string(),
            Self::Long(x) => x.to_string(),
            Self::LongLong(x) => x.to_string(),
            Self::Float(x) => x.to_string(),
            Self::Double(x) => x.to_string(),
            Self::LongDouble(x) => format!("'{}'", *x as f64),
            Self::UInt(x) => x.to_string(),
            Self::ULong(x) => x.to_string(),
            Self::ULongLong(x) => x.to_string(),
        })
    }
}

impl NumberType {
    /// Tries to increment the size of a type, by taking a bigger type.
    ///
    /// It works with the following (where M(x) means the size of the type x):
    ///
    /// ``M(Int) < M(UInt) < M(Long) < M(ULong) < M(LongLong) < M(ULongLong)``
    ///
    /// However, if the number is negative, (`signed = true`), we can't convert
    /// a signed type to an unsigned.
    ///
    /// # Note
    ///
    /// Non-integer-types cannot be incremented.
    pub(crate) const fn incr_size(&self, signed: bool) -> Option<Self> {
        #[expect(clippy::match_same_arms)]
        Some(match self {
            Self::Int if signed => Self::Long,
            Self::Int => Self::UInt,
            Self::Long if signed => Self::LongLong,
            Self::Long => Self::ULong,
            Self::LongLong if !signed => Self::ULongLong,
            Self::UInt => Self::ULong,
            Self::ULong => Self::ULongLong,
            Self::ULongLong | Self::LongLong | Self::Float | Self::Double | Self::LongDouble => {
                return None;
            }
        })
    }

    /// Checks that the type is an integer type
    pub(crate) const fn is_int(&self) -> bool {
        !matches!(self, Self::Double | Self::Float | Self::LongDouble)
    }

    /// Returns the size of the suffix of the type.
    ///
    /// # Examples
    ///
    /// - `ULongLong` corresponds to the suffix 'ull' so the suffix size is `3`.
    /// - `Int` doesn't need a suffix so the suffix size is `0`.
    pub(crate) const fn suffix_size(&self) -> usize {
        #[expect(clippy::match_same_arms)]
        match self {
            Self::Int => 0,
            Self::Long => 1,
            Self::LongLong => 2,
            Self::Float => 1,
            Self::Double => 0,
            Self::LongDouble => 1,
            Self::UInt => 1,
            Self::ULong => 2,
            Self::ULongLong => 3,
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for NumberType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Int => "int",
            Self::Long => "long",
            Self::LongLong => "long long",
            Self::Float => "float",
            Self::Double => "double",
            Self::LongDouble => "long double",
            Self::UInt => "unsigned int",
            Self::ULong => "unsigned long",
            Self::ULongLong => "unsigned long long",
        })
    }
}
