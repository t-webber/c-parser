//! Module to define escape sequence type.
//!
//! See [`EscapeSequence`].

use utils::LocationPointer;
use utils::IntoError as _;
use crate::types::LexingData;

/// Type to store the value of an escape sequence.
#[derive(Debug, Copy, Clone)]
enum EscapeSequenceValue {
    /// Escape sequences that begin with \x followed by any number hexadecimal
    /// digits.
    Hexadecimal(u32),
    /// Escape sequences that begin with a \ followed by octal digits.
    Octal(u32),
    /// Escape sequences that begin with a \u and followed by 4 hexadecimal
    /// digit.
    ///
    /// # Note
    ///
    /// This only works for codes under 0x10000.
    ShortUnicode(u32),
    /// Escape sequences that begin with a \U and followed by 8 hexadecimal
    /// digit.
    Unicode(u32),
}

impl EscapeSequenceValue {
    /// Returns the value of the escape sequence.
    const fn as_value(self) -> u32 {
        match self {
            Self::Hexadecimal(val)
            | Self::Octal(val)
            | Self::ShortUnicode(val)
            | Self::Unicode(val) => val,
        }
    }

    /// Computes the length of the escape sequence prefix.
    ///
    /// # Examples
    ///
    /// - "\U" is of length 10
    /// - "\" is of length 1.
    pub const fn len(self) -> usize {
        match self {
            Self::Unicode(_) | Self::Hexadecimal(_) | Self::ShortUnicode(_) => 2,
            Self::Octal(_) => 1,
        }
    }

    /// Gets the maximum number of digits that can appear after the prefix in
    /// the escape sequence. It corresponds to the maximum length of the
    /// underlying `String`.
    pub const fn max_len(self) -> Option<usize> {
        Some(match self {
            Self::ShortUnicode(_) => 4,
            Self::Unicode(_) => 8,
            Self::Hexadecimal(_) => return None,
            Self::Octal(_) => 3,
        })
    }
}

/// Type to store an escape sequence.
#[derive(Debug, Copy, Clone)]
pub struct EscapeSequence {
    /// Number of characters used for this escape sequence.
    char_nb: usize,
    /// Value that the escape sequence corresponds to.
    value: EscapeSequenceValue,
}

impl EscapeSequence {
    /// Creates a failure that corresponds to a wrong character being pushed in
    /// a length-determine sequence (unicode sequences).
    fn handle_invalid_char_pushed(
        self,
        lex_data: &mut LexingData,
        location: &LocationPointer,
        ch: char,
    ) -> (u32, Option<char>) {
        if let Some((max_len, prefix)) = match self.value {
            EscapeSequenceValue::Octal(_) => None,
            EscapeSequenceValue::Hexadecimal(_) => Some((1u32, 'x')),
            EscapeSequenceValue::ShortUnicode(_) => Some((4u32, 'u')),
            EscapeSequenceValue::Unicode(_) => Some((8u32, 'U')),
        } && (prefix != 'x' || self.char_nb == 0)
        {
            lex_data.push_err(
                location.to_past(self.len(), self.len()
                )
                .into_fault(format!(
                "invalid hexdigit {ch}: expected {max_len} hexdigit{} after \\{prefix} prefix, but only got {}", if max_len > 1 { "s" } else {""}, self.char_nb
            )));
        }
        (self.value.as_value(), Some(ch))
    }

    /// Increments the count of chars pushed into the sequence.
    #[expect(clippy::arithmetic_side_effects, reason = "<= 8")]
    fn increment(&mut self) {
        self.char_nb = 8.min(self.char_nb + 1);
    }

    /// Returns the length of the sequence, `\` included.
    #[expect(clippy::arithmetic_side_effects, reason = "<= 10")]
    pub const fn len(self) -> usize {
        self.value.len() + self.char_nb
    }

    /// Creartes a new hexadecimal escape sequence.
    pub const fn new_hex() -> Self {
        Self { char_nb: 0, value: EscapeSequenceValue::Hexadecimal(0) }
    }

    /// Creates a new octal escape sequence from the given number.
    ///
    /// # Panics
    ///
    /// When the provided char is not in range '0'..='9'
    pub fn new_octal(first: char) -> Self {
        debug_assert!(first.is_ascii_digit(), "invalid input: expected ascii digit, found {first}");
        Self {
            char_nb: 1,
            value: EscapeSequenceValue::Octal(hex_val(first).expect("in 0..=9").into()),
        }
    }

    /// Creates a new unicode escape sequence.
    pub const fn new_unicode(short: bool) -> Self {
        Self {
            char_nb: 0,
            value: if short {
                EscapeSequenceValue::ShortUnicode(0)
            } else {
                EscapeSequenceValue::Unicode(0)
            },
        }
    }

    /// Returns an error to inform user that \oXXX will be taken modulo 256
    #[expect(clippy::arithmetic_side_effects, reason = "1<=len<=10")]
    fn octal_too_big(self, lex_data: &mut LexingData, location: &LocationPointer) {
        let len = self.len();
        lex_data.push_err({ location.to_past(len + 1, len) }.into_warning(
            "octal value too big: exceeds 0o377: will be computed modulo 255".to_owned(),
        ));
    }

    /// Pushes a char into the [`EscapeSequence`], and returns the result if it
    /// finished processing the sequence.
    pub fn push_char(
        &mut self,
        ch: char,
        location: &LocationPointer,
        lex_data: &mut LexingData,
    ) -> Option<(u32, Option<char>)> {
        let Some(bit): Option<u32> = hex_val(ch).map(Into::into) else {
            return Some(self.handle_invalid_char_pushed(lex_data, location, ch));
        };
        let this = *self;
        match &mut self.value {
            EscapeSequenceValue::Hexadecimal(val) => {
                *val = ((*val & 0xf) << 4u32) | bit;
                if self.char_nb == 2 {
                    self.too_many_hexdigits(lex_data, location);
                }
            }
            EscapeSequenceValue::Octal(val) if bit > 7 => return Some((*val, Some(ch))),
            EscapeSequenceValue::Octal(val) => {
                *val = (*val << 3u32) | bit;
                if *val > 0xff {
                    this.octal_too_big(lex_data, location);
                    *val &= 0xff;
                }
            }
            EscapeSequenceValue::Unicode(val) | EscapeSequenceValue::ShortUnicode(val) =>
                *val = (*val << 4u32) | bit,
        }
        self.increment();
        (self.value.max_len() == Some(self.char_nb)).then(|| (self.value.as_value(), None))
    }

    /// Returns an error to inform user that \xXXX will be clamped to \xXX
    #[expect(clippy::arithmetic_side_effects, reason = "1<=len<=10")]
    fn too_many_hexdigits(self, lex_data: &mut LexingData, location: &LocationPointer) {
        let len = self.len();
        lex_data.push_err(
                        location.to_past(len+1, len
                        )
                        .into_warning("too many hexdigits after \\x: all hexdigits will be taken but only the trailing 2 will be kept".to_owned()));
    }
}

/// Returns the `u8` represented by this char, if the char is a hexdigit.
#[expect(clippy::arithmetic_side_effects, reason = "<= 16")]
fn hex_val(ch: char) -> Option<u8> {
    match u8::try_from(ch) {
        Ok(bit @ b'0'..=b'9') => Some(bit - b'0'),
        Ok(bit @ b'a'..=b'f') => Some(bit - b'a' + 10),
        Ok(bit @ b'A'..=b'F') => Some(bit - b'A' + 10),
        _ => None,
    }
}
