#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EscapeSequence {
    Hexadecimal(String),
    Octal(String),
    ShortUnicode(String), // bellow 10000 hexadecimal (4 hex digits)
    Unicode(String),      // all hexadecimal (8 hex digits)
}

impl EscapeSequence {
    pub const fn is_hexa(&self) -> bool {
        matches!(self, Self::Hexadecimal(_) | Self::ShortUnicode(_))
    }

    pub const fn is_octal(&self) -> bool {
        matches!(self, Self::Octal(_))
    }

    pub const fn max_len(&self) -> usize {
        match self {
            Self::ShortUnicode(_) => 4,
            Self::Unicode(_) => 8,
            Self::Hexadecimal(_) => 2,
            Self::Octal(_) => 3,
        }
    }

    pub const fn prefix(&self) -> &'static str {
        match self {
            Self::ShortUnicode(_) => "\\u",
            Self::Unicode(_) => "\\U",
            Self::Hexadecimal(_) => "\\x",
            Self::Octal(_) => "\\",
        }
    }

    pub const fn repr(&self) -> &'static str {
        match self {
            Self::Hexadecimal(_) => "hexadecimal",
            Self::Octal(_) => "octal",
            Self::ShortUnicode(_) => "short unicode",
            Self::Unicode(_) => "unicode",
        }
    }

    pub const fn value(&self) -> &String {
        match self {
            Self::ShortUnicode(value)
            | Self::Unicode(value)
            | Self::Hexadecimal(value)
            | Self::Octal(value) => value,
        }
    }

    pub fn value_mut(&mut self) -> &mut String {
        match self {
            Self::Unicode(value)
            | Self::ShortUnicode(value)
            | Self::Hexadecimal(value)
            | Self::Octal(value) => value,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EscapeStatus {
    Sequence(EscapeSequence),
    Single,
    False,
}
