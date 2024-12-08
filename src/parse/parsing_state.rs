use super::Symbol;
use crate::errors::compile::Errors;

const NULL: char = '\0';

#[derive(Default, Debug)]
pub struct ParsingState {
    pub errors: Errors,
    pub escape: bool,
    pub p_state: StateState,
    // p_state = Symbol
    first: char,
    second: char,
    third: char,
    // p_state = Identifier
    pub double_quote: bool,
    pub literal: String,
    pub single_quote: TriBool,
}

impl ParsingState {
    fn clear(&mut self) {
        self.first = NULL;
        self.second = NULL;
        self.third = NULL;
        self.double_quote = false;
        self.single_quote = TriBool::False;
        self.escape = false;
        self.literal.clear();
    }

    pub const fn is_empty(&self) -> bool {
        self.first == NULL && self.second == NULL && self.third == NULL
    }

    pub fn is_number(&self) -> bool {
        let mut chars = self.literal.chars();
        chars.next().map_or_else(|| false, char::is_numeric)
            && chars.all(|ch| ch.is_numeric() || ch == '.' || ch == '_')
    }

    pub fn push(&mut self, value: char) -> Option<Symbol> {
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

    pub fn try_to_operator(&mut self) -> Option<Symbol> {
        use Symbol as OT;
        let (nb_consumed, operator): (u32, _) = match (self.first, self.second, self.third) {
            ('<', '<', '=') => (3, Some(OT::ShiftLeftAssign)),
            ('>', '>', '=') => (3, Some(OT::ShiftRightAssign)),
            ('-', '>', _) => (2, Some(OT::Arrow)),
            ('+', '+', _) => (2, Some(OT::Increment)),
            ('-', '-', _) => (2, Some(OT::Decrement)),
            ('<', '<', _) => (2, Some(OT::ShiftLeft)),
            ('>', '>', _) => (2, Some(OT::ShiftRight)),
            ('&', '&', _) => (2, Some(OT::LogicalAnd)),
            ('|', '|', _) => (2, Some(OT::LogicalOr)),
            ('<', '=', _) => (2, Some(OT::Le)),
            ('>', '=', _) => (2, Some(OT::Ge)),
            ('=', '=', _) => (2, Some(OT::Equal)),
            ('!', '=', _) => (2, Some(OT::Different)),
            ('+', '=', _) => (2, Some(OT::AddAssign)),
            ('-', '=', _) => (2, Some(OT::SubAssign)),
            ('*', '=', _) => (2, Some(OT::MulAssign)),
            ('/', '=', _) => (2, Some(OT::DivAssign)),
            ('%', '=', _) => (2, Some(OT::ModAssign)),
            ('&', '=', _) => (2, Some(OT::AndAssign)),
            ('|', '=', _) => (2, Some(OT::OrAssign)),
            ('^', '=', _) => (2, Some(OT::XorAssign)),
            ('+', _, _) => (1, Some(OT::Plus)),
            ('-', _, _) => (1, Some(OT::Minus)),
            ('(', _, _) => (1, Some(OT::ParenthesisOpen)),
            (')', _, _) => (1, Some(OT::ParenthesisClose)),
            ('[', _, _) => (1, Some(OT::BracketOpen)),
            (']', _, _) => (1, Some(OT::BracketClose)),
            ('.', _, _) => (1, Some(OT::Dot)),
            ('{', _, _) => (1, Some(OT::BraceOpen)),
            ('}', _, _) => (1, Some(OT::BraceClose)),
            ('~', _, _) => (1, Some(OT::BitwiseNot)),
            ('!', _, _) => (1, Some(OT::LogicalNot)),
            ('*', _, _) => (1, Some(OT::Star)),
            ('&', _, _) => (1, Some(OT::Ampercent)),
            ('%', _, _) => (1, Some(OT::Modulo)),
            ('/', _, _) => (1, Some(OT::Divide)),
            ('>', _, _) => (1, Some(OT::Gt)),
            ('<', _, _) => (1, Some(OT::Lt)),
            ('=', _, _) => (1, Some(OT::Assign)),
            ('|', _, _) => (1, Some(OT::BitwiseOr)),
            ('^', _, _) => (1, Some(OT::BitwiseXor)),
            (',', _, _) => (1, Some(OT::Comma)),
            ('?', _, _) => (1, Some(OT::Interrogation)),
            (':', _, _) => (1, Some(OT::Colon)),
            (NULL, NULL, NULL) => (0, None),
            _ => panic!(
                "This is not meant to happen. Some unsupported symbols were found in the operator part of the p_state. ParsingState: {self:?}"
            ),
        };
        match nb_consumed {
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
        operator
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum StateState {
    Identifier,
    #[default]
    None,
    Symbol,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum TriBool {
    #[default]
    False,
    Intermediate,
    True,
}
