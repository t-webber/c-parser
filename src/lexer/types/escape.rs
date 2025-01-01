//! Module to define escape sequence type.
//!
//! See [`EscapeSequence`].

/// Type to store an escape sequence.
///
/// Escape sequence start with a backslash, and then contains characters. The
/// combination of all these make 1 char.
#[derive(Debug, PartialEq, Eq)]
pub enum EscapeSequence {
    /// Escape sequences that begin with \x followed by 4 hexadecimal digits.
    Hexadecimal(String),
    /// Escape sequences that begin with a \ followed by octal digits.
    Octal(String),
    /// Escape sequences that begin with a \u and followed by 4 hexadecimal
    /// digit.
    ///
    /// # Note
    ///
    /// This only works for codes under 0x10000.
    ShortUnicode(String),
    /// Escape sequences that begin with a \U and followed by 8 hexadecimal
    /// digit.
    Unicode(String),
}

impl EscapeSequence {
    /// Checks if the escape sequence is octal.
    pub const fn is_octal(&self) -> bool {
        matches!(self, Self::Octal(_))
    }

    /// Gets the maximum number of digits that can appear after the prefix in
    /// the escape sequence. It corresponds to the maximum length of the
    /// underlying `String`.
    pub const fn max_len(&self) -> usize {
        match self {
            Self::ShortUnicode(_) => 4,
            Self::Unicode(_) => 8,
            Self::Hexadecimal(_) => 2,
            Self::Octal(_) => 3,
        }
    }

    /// Gives a pretty representation of the escape type.
    pub const fn repr(&self) -> &'static str {
        match self {
            Self::Hexadecimal(_) => "hexadecimal",
            Self::Octal(_) => "octal",
            Self::ShortUnicode(_) => "short unicode",
            Self::Unicode(_) => "unicode",
        }
    }

    /// Gives a mutable reference of the underlying `String`.
    pub fn value_mut(&mut self) -> &mut String {
        match self {
            Self::Unicode(value)
            | Self::ShortUnicode(value)
            | Self::Hexadecimal(value)
            | Self::Octal(value) => value,
        }
    }
}
