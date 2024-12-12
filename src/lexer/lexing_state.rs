use super::{Symbol, Token};
use crate::errors::compile::CompileError;
use core::mem;

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
            Self::SmallUnicode(_) => 4,
            Self::BigUnicode(_) => 8,
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
                EscapeSequence::ShortUnicode(value)
                | EscapeSequence::Unicode(value)
                | EscapeSequence::Hexadecimal(value)
                | EscapeSequence::Octal(value),
            ) => value,
            Self::Trivial(_) => {
                panic!("Called get_unsafe_sequence_value_mut without checking if authorised")
            }
        }
    }
}

#[derive(Debug)]
pub struct ParsingState {
    errors: Vec<CompileError>,
    tokens: Vec<Token>,
    pub failed: bool,
    pub start_of_line: bool,
    /// Block comments
    pub comments: CommentStatus,
    pub escape: EscapeStatus,
    /* lex_state = Symbol */
    first: char,
    second: char,
    third: char,
    /* lex_state = Identifier */
    pub double_quote: bool,
    pub literal: String,
    pub single_quote: CharStatus,
}

impl ParsingState {
    pub fn clear_all_err(&mut self) {
        *self = Self::new();
    }

    pub fn clear_last(&mut self) {
        if self.third != NULL {
            self.third = NULL;
        } else if self.second != NULL {
            self.second = NULL;
        } else if self.first != NULL {
            self.first = NULL;
        } else {
            panic!("Called clear_last without checking that last exists.");
        }
    }

    pub fn take_errors(&mut self) -> Vec<CompileError> {
        mem::take(&mut self.errors)
    }

    pub fn take_tokens(&mut self) -> Vec<Token> {
        mem::take(&mut self.tokens)
    }

    pub const fn is_empty(&self) -> bool {
        self.first == NULL && self.second == NULL && self.third == NULL
    }

    pub fn is_number(&self) -> bool {
        self.literal.chars().next().unwrap_or(NULL).is_ascii_digit()
    }

    pub fn is_hex(&self) -> bool {
        self.literal.starts_with("0x")
    }

    pub fn last_literal_char(&self) -> Option<char> {
        self.literal.chars().last()
    }

    pub const fn last_symbol(&self) -> Option<char> {
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

    pub const fn new() -> Self {
        Self {
            failed: false,
            start_of_line: true,
            errors: vec![],
            tokens: vec![],
            escape: EscapeStatus::Trivial(false),
            comments: CommentStatus::False,
            first: NULL,
            second: NULL,
            third: NULL,
            double_quote: false,
            literal: String::new(),
            single_quote: CharStatus::Closed,
        }
    }

    pub fn pop_token(&mut self) -> Option<Token> {
        self.tokens.pop()
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
        let is_error = error.is_error();
        self.errors.push(error);
        if is_error {
            self.clear_all_err();
            self.failed = true;
        }
    }

    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
        self.literal.clear();
    }

    pub fn try_to_operator(&mut self) -> Option<(usize, Symbol)> {
        let result = match (self.first, self.second, self.third) {
            ('<', '<', '=') => Some((3, Symbol::ShiftLeftAssign)),
            ('>', '>', '=') => Some((3, Symbol::ShiftRightAssign)),
            ('-', '>', _) => Some((2, Symbol::Arrow)),
            ('+', '+', _) => Some((2, Symbol::Increment)),
            ('-', '-', _) => Some((2, Symbol::Decrement)),
            ('<', '<', _) => Some((2, Symbol::ShiftLeft)),
            ('>', '>', _) => Some((2, Symbol::ShiftRight)),
            ('&', '&', _) => Some((2, Symbol::LogicalAnd)),
            ('|', '|', _) => Some((2, Symbol::LogicalOr)),
            ('<', '=', _) => Some((2, Symbol::Le)),
            ('>', '=', _) => Some((2, Symbol::Ge)),
            ('=', '=', _) => Some((2, Symbol::Equal)),
            ('!', '=', _) => Some((2, Symbol::Different)),
            ('+', '=', _) => Some((2, Symbol::AddAssign)),
            ('-', '=', _) => Some((2, Symbol::SubAssign)),
            ('*', '=', _) => Some((2, Symbol::MulAssign)),
            ('/', '=', _) => Some((2, Symbol::DivAssign)),
            ('%', '=', _) => Some((2, Symbol::ModAssign)),
            ('&', '=', _) => Some((2, Symbol::AndAssign)),
            ('|', '=', _) => Some((2, Symbol::OrAssign)),
            ('^', '=', _) => Some((2, Symbol::XorAssign)),
            ('+', _, _) => Some((1, Symbol::Plus)),
            ('-', _, _) => Some((1, Symbol::Minus)),
            ('(', _, _) => Some((1, Symbol::ParenthesisOpen)),
            (')', _, _) => Some((1, Symbol::ParenthesisClose)),
            ('[', _, _) => Some((1, Symbol::BracketOpen)),
            (']', _, _) => Some((1, Symbol::BracketClose)),
            ('.', _, _) => Some((1, Symbol::Dot)),
            ('{', _, _) => Some((1, Symbol::BraceOpen)),
            ('}', _, _) => Some((1, Symbol::BraceClose)),
            ('~', _, _) => Some((1, Symbol::BitwiseNot)),
            ('!', _, _) => Some((1, Symbol::LogicalNot)),
            ('*', _, _) => Some((1, Symbol::Star)),
            ('&', _, _) => Some((1, Symbol::Ampercent)),
            ('%', _, _) => Some((1, Symbol::Modulo)),
            ('/', _, _) => Some((1, Symbol::Divide)),
            ('>', _, _) => Some((1, Symbol::Gt)),
            ('<', _, _) => Some((1, Symbol::Lt)),
            ('=', _, _) => Some((1, Symbol::Assign)),
            ('|', _, _) => Some((1, Symbol::BitwiseOr)),
            ('^', _, _) => Some((1, Symbol::BitwiseXor)),
            (',', _, _) => Some((1, Symbol::Comma)),
            ('?', _, _) => Some((1, Symbol::Interrogation)),
            (':', _, _) => Some((1, Symbol::Colon)),
            (';', _, _) => Some((1, Symbol::SemiColon)),
            (NULL, NULL, NULL) => None,
            _ => panic!(
                "This is not meant to happen. Some unsupported symbols were found in the operator part of the lex_state. ParsingState: {self:?}"
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

#[derive(Debug, Default, PartialEq, Eq)]
pub enum StateState {
    Identifier,
    #[default]
    None,
    Symbol,
}
