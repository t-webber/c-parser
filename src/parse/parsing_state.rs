use super::Symbol;
use crate::errors::{
    compile::{CompileError, Errors},
    location::Location,
};

const NULL: char = '\0';

#[derive(Default, Debug, PartialEq, Eq)]
pub enum CharStatus {
    #[default]
    Closed,
    Opened,
    Written,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommentStatus {
    True,
    False,
    Star,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EscapeSequence {
    Hexadecimal(String),
    Octal(String),
    Unicode(String),
}

impl EscapeSequence {
    pub const fn is_hexa(&self) -> bool {
        matches!(self, Self::Hexadecimal(_) | Self::Unicode(_))
    }

    pub const fn is_octal(&self) -> bool {
        matches!(self, Self::Octal(_))
    }

    pub const fn max_len(&self) -> usize {
        match self {
            Self::Unicode(_) => 4,
            Self::Hexadecimal(_) => 2,
            Self::Octal(_) => 3,
        }
    }

    pub const fn prefix(&self) -> &'static str {
        match self {
            Self::Unicode(_) => "\\u",
            Self::Hexadecimal(_) => "\\x",
            Self::Octal(_) => "\\",
        }
    }

    pub const fn value(&self) -> &String {
        match self {
            Self::Unicode(value) | Self::Hexadecimal(value) | Self::Octal(value) => value,
        }
    }

    pub fn value_mut(&mut self) -> &mut String {
        match self {
            Self::Unicode(value) | Self::Hexadecimal(value) | Self::Octal(value) => value,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EscapeStatus {
    Sequence(EscapeSequence),
    Trivial(bool),
}

impl EscapeStatus {
    pub fn get_unsafe_sequence(&self) -> EscapeSequence {
        if let Self::Sequence(seq) = self {
            seq.to_owned()
        } else {
            panic!("Called get_unsafe_sequence without checking if authorised")
        }
    }
    pub fn get_unsafe_sequence_mut(&mut self) -> &mut EscapeSequence {
        if let Self::Sequence(seq) = self {
            seq
        } else {
            panic!("Called get_unsafe_sequence without checking if authorised")
        }
    }
    pub fn get_unsafe_sequence_value_mut(&mut self) -> &mut String {
        match self {
            Self::Sequence(
                EscapeSequence::Unicode(value)
                | EscapeSequence::Hexadecimal(value)
                | EscapeSequence::Octal(value),
            ) => value,
            _ => panic!("Called get_unsafe_sequence_value_mut without checking if authorised"),
        }
    }
}

#[derive(Debug)]
pub struct ParsingState {
    errors: Errors,
    /// Block comments
    pub comments: CommentStatus,
    pub escape: EscapeStatus,
    pub initial_location: Location,
    // p_state = Symbol
    first: char,
    second: char,
    third: char,
    // p_state = Identifier
    pub double_quote: bool,
    pub literal: String,
    pub single_quote: CharStatus,
}

impl ParsingState {
    pub fn clear(&mut self) {
        self.first = NULL;
        self.second = NULL;
        self.third = NULL;
        self.double_quote = false;
        self.single_quote = CharStatus::Closed;
        self.escape = EscapeStatus::Trivial(false);
        self.literal.clear();
    }

    pub fn clear_last(&mut self) {
        if self.third != NULL {
            self.third = NULL
        } else if self.second != NULL {
            self.second = NULL
        } else if self.first != NULL {
            self.first = NULL
        } else {
            panic!("Called clear_last without checking that last exists.")
        }
    }

    pub fn get_errors(self) -> Vec<CompileError> {
        self.errors
    }

    pub const fn is_empty(&self) -> bool {
        self.first == NULL && self.second == NULL && self.third == NULL
    }

    pub fn is_number(&self) -> bool {
        let mut chars = self.literal.chars();
        chars.next().map_or_else(|| false, char::is_numeric)
            && chars.all(|ch| ch.is_numeric() || ch == '.' || ch == '_')
    }

    pub const fn last(&self) -> Option<char> {
        if self.third == NULL {
            if self.second == NULL {
                if self.first == NULL {
                    None
                } else {
                    Some(self.first)
                }
            } else {
                Some(self.second)
            }
        } else {
            Some(self.first)
        }
    }

    pub fn push(&mut self, value: char) -> Option<(usize, Symbol)> {
        let op = if self.third == NULL {
            None
        } else {
            self.try_to_operator()
        };
        if self.first == NULL {
            self.first = value;
        } else if self.second == NULL {
            self.second = value;
        } else if self.third == NULL {
            self.third = value;
        } else {
            panic!("This is not meant to happen. Called try_operator on none empty self, and no operator was returned. ParsingState: {self:?}");
        }
        op
    }

    pub fn push_err(&mut self, error: CompileError) {
        self.errors.push(error);
        self.clear();
    }

    pub fn try_to_operator(&mut self) -> Option<(usize, Symbol)> {
        use Symbol as S;
        let result = match (self.first, self.second, self.third) {
            ('<', '<', '=') => Some((3, S::ShiftLeftAssign)),
            ('>', '>', '=') => Some((3, S::ShiftRightAssign)),
            ('-', '>', _) => Some((2, S::Arrow)),
            ('+', '+', _) => Some((2, S::Increment)),
            ('-', '-', _) => Some((2, S::Decrement)),
            ('<', '<', _) => Some((2, S::ShiftLeft)),
            ('>', '>', _) => Some((2, S::ShiftRight)),
            ('&', '&', _) => Some((2, S::LogicalAnd)),
            ('|', '|', _) => Some((2, S::LogicalOr)),
            ('<', '=', _) => Some((2, S::Le)),
            ('>', '=', _) => Some((2, S::Ge)),
            ('=', '=', _) => Some((2, S::Equal)),
            ('!', '=', _) => Some((2, S::Different)),
            ('+', '=', _) => Some((2, S::AddAssign)),
            ('-', '=', _) => Some((2, S::SubAssign)),
            ('*', '=', _) => Some((2, S::MulAssign)),
            ('/', '=', _) => Some((2, S::DivAssign)),
            ('%', '=', _) => Some((2, S::ModAssign)),
            ('&', '=', _) => Some((2, S::AndAssign)),
            ('|', '=', _) => Some((2, S::OrAssign)),
            ('^', '=', _) => Some((2, S::XorAssign)),
            ('+', _, _) => Some((1, S::Plus)),
            ('-', _, _) => Some((1, S::Minus)),
            ('(', _, _) => Some((1, S::ParenthesisOpen)),
            (')', _, _) => Some((1, S::ParenthesisClose)),
            ('[', _, _) => Some((1, S::BracketOpen)),
            (']', _, _) => Some((1, S::BracketClose)),
            ('.', _, _) => Some((1, S::Dot)),
            ('{', _, _) => Some((1, S::BraceOpen)),
            ('}', _, _) => Some((1, S::BraceClose)),
            ('~', _, _) => Some((1, S::BitwiseNot)),
            ('!', _, _) => Some((1, S::LogicalNot)),
            ('*', _, _) => Some((1, S::Star)),
            ('&', _, _) => Some((1, S::Ampercent)),
            ('%', _, _) => Some((1, S::Modulo)),
            ('/', _, _) => Some((1, S::Divide)),
            ('>', _, _) => Some((1, S::Gt)),
            ('<', _, _) => Some((1, S::Lt)),
            ('=', _, _) => Some((1, S::Assign)),
            ('|', _, _) => Some((1, S::BitwiseOr)),
            ('^', _, _) => Some((1, S::BitwiseXor)),
            (',', _, _) => Some((1, S::Comma)),
            ('?', _, _) => Some((1, S::Interrogation)),
            (':', _, _) => Some((1, S::Colon)),
            (';', _, _) => Some((1, S::SemiColon)),
            (NULL, NULL, NULL) => None,
            _ => panic!(
                "This is not meant to happen. Some unsupported symbols were found in the operator part of the p_state. ParsingState: {self:?}"
            ),
        };

        if let Some((nb_consumed, _)) = &result {
            match *nb_consumed {
            0 => (), // two consecutive litterals
            1 => {
                self.first = self.second;
                self.second = self.third;
                self.third = NULL;
            }
            2 => {
                self.first = self.third;
                self.second = NULL;
                self.third = NULL;
            }
            3 => {
                self.first = NULL;
                self.second = NULL;
                self.third = NULL;
            }
            _ => panic!("his is not meant to happen. nb_consumed is defined only be having values of 0, 1, 2 or 3, not {nb_consumed}"),
        };
        };
        result
    }
}

impl From<Location> for ParsingState {
    fn from(value: Location) -> Self {
        Self {
            errors: vec![],
            escape: EscapeStatus::Trivial(false),
            comments: CommentStatus::False,
            initial_location: value,
            first: NULL,
            second: NULL,
            third: NULL,
            double_quote: false,
            literal: String::new(),
            single_quote: CharStatus::Closed,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum StateState {
    Identifier,
    #[default]
    None,
    Symbol,
}
