use crate::errors::api::Location;
use crate::lexer::api::Symbol;
use crate::lexer::types::api::LexingData;

const NULL: char = '\0';

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolState {
    first: char,
    second: char,
    third: char,
}

impl SymbolState {
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

    fn handle_digraphs_trigraphs(&mut self) -> Option<(String, usize, bool)> {
        let symbols = (self.first, self.second, self.third);
        let (graph, is_trigraph) = match symbols {
            ('?', '?', '=') => (Some('#'), true),
            ('?', '?', '/') => (Some('\\'), true),
            ('?', '?', '\'') => (Some('^'), true),
            ('?', '?', '(') => (Some('['), true),
            ('?', '?', ')') => (Some(']'), true),
            ('?', '?', '!') => (Some('|'), true),
            ('?', '?', '<') => (Some('{'), true),
            ('?', '?', '>') => (Some('}'), true),
            ('?', '?', '-') => (Some('~'), true),
            ('<', ':', _) => (Some('['), false),
            (':', '>', _) => (Some(']'), false),
            ('<', '%', _) => (Some('{'), false),
            ('%', '>', _) => (Some('}'), false),
            ('%', ':', _) => {
                return Some((
                    "Found invalid character '#', found by replacing digraph '%:'.".to_owned(),
                    2,
                    true,
                ));
            }
            _ => (None, false),
        };
        if let Some(symbol) = graph {
            if is_trigraph {
                let msg = format!(
                    "Trigraphs are deprecated in C23. Please remove them: replace '{}{}{}' by '{symbol}'.",
                    self.first, self.second, self.third
                );
                self.first = NULL;
                self.second = NULL;
                self.third = NULL;
                return Some((msg, 3, false));
            }
            self.first = symbol;
            self.second = self.third;
            self.third = NULL;
        }
        None
    }

    pub const fn is_empty(&self) -> bool {
        self.first == NULL && self.second == NULL && self.third == NULL
    }

    pub const fn is_trigraph(&self) -> bool {
        matches!(
            (self.first, self.second, self.third),
            ('?', '?', NULL) | (_, '?', '?')
        )
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

    pub const fn new(ch: char) -> Self {
        Self {
            first: ch,
            second: NULL,
            third: NULL,
        }
    }

    pub fn push(
        &mut self,
        value: char,
        lex_data: &mut LexingData,
        location: &Location,
    ) -> Option<(usize, Symbol)> {
        let op = if self.third == NULL {
            None
        } else {
            self.try_to_operator(lex_data, location)
        };
        if self.first == NULL {
            self.first = value;
        } else if self.second == NULL {
            self.second = value;
        } else if self.third == NULL {
            self.third = value;
        } else {
            panic!(
                "This is not meant to happen. Called try_operator on none empty self, and no operator was returned. LexingData: {self:?}"
            );
        }
        op
    }

    pub fn try_to_operator(
        &mut self,
        lex_data: &mut LexingData,
        location: &Location,
    ) -> Option<(usize, Symbol)> {
        if let Some((msg, len, error)) = self.handle_digraphs_trigraphs() {
            let new_location = location.to_owned().into_past_with_length(len);
            if error {
                lex_data.push_err(new_location.to_error(msg));
            } else {
                lex_data.push_err(new_location.to_warning(msg));
            }
        }
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
            ('&', _, _) => Some((1, Symbol::Ampersand)),
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
                0 => (), // two consecutive literals
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
                _ => panic!(
                    "This is not meant to happen. `nb_consumed` is defined only be having values of 0, 1, 2 or 3, not {nb_consumed}"
                ),
            };
        }
        result
    }
}
