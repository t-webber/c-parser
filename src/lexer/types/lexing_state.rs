use super::tokens_types::Symbol;
use core::{mem, str::pattern::Pattern};

const NULL: char = '\0';

#[derive(Debug, PartialEq, Eq)]
pub enum CommentStatus {
    True,
    False,
    Star,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Ident(String);

impl Ident {
    pub fn contains<P: Pattern>(&self, pat: P) -> bool {
        self.0.contains(pat)
    }

    pub fn first(&self) -> Option<char> {
        self.0.chars().next()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_number(&self) -> bool {
        self.first().unwrap_or('x').is_ascii_digit()
    }

    pub fn last_is_exp(&self) -> bool {
        self.is_number()
            && match self.0.chars().last() {
                Some('p' | 'P') => self.0.starts_with("0x"),
                Some('e' | 'E') => !self.0.starts_with("0x"), // if the number expression starts with 0 and contains an exponent, the number is considered decimal, not octal.
                Some(_) | None => false,
            }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, ch: char) {
        self.0.push(ch);
    }

    pub fn take_value(&mut self) -> String {
        mem::take(&mut self.0)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum LexingStatus {
    #[default]
    StartOfLine,
    Unset,
    Symbols(SymbolStatus),
    Identifier(Ident),
    Char(Option<char>),
    Str(String),
    Comment(CommentStatus),
}

impl LexingStatus {
    pub const fn repr(&self) -> &'static str {
        match self {
            Self::StartOfLine => "start of line",
            Self::Unset => "no context",
            Self::Symbols(_) => "symbols",
            Self::Identifier(_) => "identifier",
            Self::Char(_) => "char",
            Self::Str(_) => "string",
            Self::Comment(_) => "comment",
        }
    }

    pub const fn symbol(&self) -> Option<&SymbolStatus> {
        if let Self::Symbols(symb) = self {
            Some(symb)
        } else {
            None
        }
    }
    pub fn clear_last_symbol(&mut self) {
        if let Self::Symbols(symb) = self {
            symb.clear_last();
        } else {
            panic!("Didn't check if allowed before calling on symbol")
        }
    }

    pub fn new_ident(&mut self, ch: char) {
        *self = Self::Identifier(Ident(String::from(ch)));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolStatus {
    first: char,
    second: char,
    third: char,
}

impl SymbolStatus {
    pub const fn clear_last(&mut self) {
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

    pub const fn is_empty(&self) -> bool {
        self.first == NULL && self.second == NULL && self.third == NULL
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
            panic!("This is not meant to happen. Called try_operator on none empty self, and no operator was returned. LexingData: {self:?}");
        }
        op
    }

    pub fn try_to_operator(&mut self) -> Option<(usize, Symbol)> {
        let result = match (self.first, self.second, self.third) {
            ('<', '<', '=') => Some((3, Symbol::LeftShiftAssign)),
            ('>', '>', '=') => Some((3, Symbol::RightShiftAssign)),
            ('-', '>', _) => Some((2, Symbol::Arrow)),
            ('+', '+', _) => Some((2, Symbol::Increment)),
            ('-', '-', _) => Some((2, Symbol::Decrement)),
            ('<', '<', _) => Some((2, Symbol::LeftShift)),
            ('>', '>', _) => Some((2, Symbol::RightShift)),
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
                "This is not meant to happen. Some unsupported symbols were found in the operator part of the lex_data. LexingData: {self:?}"
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

    pub const fn new(ch: char) -> Self {
        Self {
            first: ch,
            second: NULL,
            third: NULL,
        }
    }
}
