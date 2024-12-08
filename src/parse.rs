#![allow(dead_code)]

use crate::errors::{CompileError, Errors, Res};
use crate::location::{self, Location};
use crate::{
    to_error,
    tree::{Literal, Node},
};
use core::mem::take;

const NULL: char = '\0';

#[derive(Debug)]
enum Symbol {
    // one character
    Ampercent,
    Assign,
    BitwiseNot,
    BitwiseOr,
    BitwiseXor,
    BraceClose,
    BraceOpen,
    BracketClose,
    BracketOpen,
    Colon,
    Comma,
    Divide,
    Dot,
    Gt,
    Interrogation,
    LogicalNot,
    Lt,
    Minus,
    Modulo,
    ParenthesisClose,
    ParenthesisOpen,
    Plus,
    Star,
    // two characters
    AddAssign,
    AndAssign,
    Arrow,
    Decrement,
    Different,
    DivAssign,
    Equal,
    Ge,
    Increment,
    Le,
    LogicalAnd,
    LogicalOr,
    ModAssign,
    MulAssign,
    OrAssign,
    ShiftLeft,
    ShiftRight,
    SubAssign,
    XorAssign,
    // three characters
    ShiftLeftAssign,
    ShiftRightAssign,
}

#[derive(Debug)]
enum Token {
    Char(char),
    Identifier(String),
    Number(String),
    Str(String),
    Symbol(Symbol),
}

#[derive(Default, Debug)]
struct ParsingState {
    errors: Errors,
    escape: bool,
    p_state: StateState,
    // p_state = Symbol
    first: char,
    second: char,
    third: char,
    // p_state = Identifier
    double_quote: bool,
    literal: String,
    single_quote: TriBool,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum StateState {
    Identifier,
    #[default]
    None,
    Symbol,
}

#[derive(Default, Debug, PartialEq, Eq)]
enum TriBool {
    #[default]
    False,
    Intermediate,
    True,
}

impl ParsingState {
    const fn is_empty(&self) -> bool {
        self.first == NULL && self.second == NULL && self.third == NULL
    }

    fn is_number(&self) -> bool {
        let mut chars = self.literal.chars();
        chars.next().map_or_else(|| false, |ch| ch.is_numeric())
            && chars.all(|ch| ch.is_numeric() || ch == '.' || ch == '_')
    }

    fn push(&mut self, value: char) -> Option<Symbol> {
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

    fn try_to_operator(&mut self) -> Option<Symbol> {
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

    fn clear(&mut self) {
        self.first = NULL;
        self.second = NULL;
        self.third = NULL;
        self.double_quote = false;
        self.single_quote = TriBool::False;
        self.escape = false;
        self.literal.clear();
    }
}

fn end_both(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    end_operator(p_state, tokens);
    end_literal(p_state, tokens, location);
}

fn end_literal(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    if !p_state.literal.is_empty() {
        let mut chars = p_state.literal.chars();
        let first = chars.next().unwrap();
        if first.is_numeric() {
            if chars.all(|ch| ch.is_numeric() || ch == '_' || ch == '.') {
                tokens.push(Token::Number(take(&mut p_state.literal)));
            } else {
                p_state.literal.clear();
                p_state.errors.push(to_error!(location, "Number immediatly followed by character. Literals can only start with alphabetic characters. Did you forget a space?"));
            };
        } else if first.is_alphabetic() {
            tokens.push(Token::Identifier(take(&mut p_state.literal)));
        } else {
            p_state.literal.clear();
            p_state.errors.push(to_error!(
                location,
                "Literals must start with a alphanumeric character, found symbol {first}."
            ));
        }
    }
}

fn end_operator(p_state: &mut ParsingState, tokens: &mut Vec<Token>) {
    let mut idx: usize = 0;
    while !p_state.is_empty() && idx <= 2 {
        idx += 1;
        if let Some(operator) = p_state.try_to_operator() {
            tokens.push(Token::Symbol(operator));
        } else {
            panic!(
                "This can't happen, as p_state is not empty! ParsingState: {:?}",
                &p_state
            );
        }
    }
    assert!(p_state.is_empty(), "Not possible: executing 3 times the conversion, with stritcly decreasing number of non empty elements! This can't happen. ParsingState: {:?}", &p_state);
}

fn end_string(p_state: &mut ParsingState, tokens: &mut Vec<Token>) {
    if !p_state.literal.is_empty() {
        tokens.push(Token::Str(take(&mut p_state.literal)));
    }
    assert!(p_state.literal.is_empty(), "Not possible: The string was just cleared, except if i am stupid and take doesn't clear ??!! ParsingState:{:?}", &p_state);
}

fn handle_escaped_character(ch: char, p_state: &mut ParsingState, location: &Location) {
    if p_state.p_state == StateState::None
        || p_state.p_state == StateState::Symbol
        || (!p_state.double_quote && p_state.single_quote != TriBool::True)
    {
        p_state.errors.push(to_error!(
            location,
            "\\ escape character can only be used inside a string or char to espace a character."
        ));
    } else {
        match ch {
            'n' => p_state.literal.push('\n'),
            't' => p_state.literal.push('\t'),
            _ => p_state
                .errors
                .push(to_error!(location, "Character {ch} can not be escaped.")),
        }
    }
    p_state.escape = false;
}

fn handle_double_quotes(p_state: &mut ParsingState, tokens: &mut Vec<Token>, location: &Location) {
    if p_state.double_quote {
        end_string(p_state, tokens);
        p_state.double_quote = false;
    } else {
        end_both(p_state, tokens, location);
        p_state.double_quote = true;
    }
}

fn handle_single_quotes(p_state: &mut ParsingState, location: &Location) {
    match p_state.single_quote {
        TriBool::False => p_state.single_quote = TriBool::True,
        TriBool::Intermediate => p_state.single_quote = TriBool::False,
        TriBool::True => p_state.errors.push(to_error!(
            location,
            "A char must contain exactly one element, but none where found. Did you mean '\\''?"
        )),
    }
}

fn get_tokens(expression: &str, location: &mut Location) -> Res<Vec<Token>> {
    let mut tokens = vec![];
    let mut p_state = ParsingState::default();
    for ch in expression.chars() {
        // println!("ParsingState = {:?}\tTokens = {:?}", &p_state, &tokens);
        match ch {
            /* Escape character */
            _ if p_state.escape => handle_escaped_character(ch, &mut p_state, location),
            '\\' => p_state.escape = true,

            /* Static strings and chars*/
            // open/close
            '\'' => handle_single_quotes(&mut p_state, location),
            '\"' => handle_double_quotes(&mut p_state, &mut tokens, location),
            // middle
            _ if p_state.single_quote == TriBool::Intermediate => p_state.errors.push(to_error!(
                location,
                "A char must contain only one character"
            )),
            _ if p_state.single_quote == TriBool::True => {
                tokens.push(Token::Char(ch));
                p_state.single_quote = TriBool::Intermediate;
            }
            _ if p_state.double_quote => p_state.literal.push(ch),

            // Operator symbols
            '+' | '-' | '(' | ')' | '[' | ']' | '{' | '}' | '~' | '!' | '*' | '&' | '%' | '/'
            | '>' | '<' | '=' | '|' | '^' | ',' | '?' | ':' => {
                end_literal(&mut p_state, &mut tokens, location);
                if let Some(operator) = p_state.push(ch) {
                    tokens.push(Token::Symbol(operator));
                }
            }
            '.' if !p_state.is_number() => {
                end_literal(&mut p_state, &mut tokens, location);
                if let Some(operator) = p_state.push(ch) {
                    tokens.push(Token::Symbol(operator));
                }
            }

            // Whitespace: end of everyone
            _ if ch.is_whitespace() => {
                end_both(&mut p_state, &mut tokens, location);
            }

            // Whitespace: end of everyone
            _ if ch.is_alphanumeric() || ch == '_' || ch == '.' => {
                end_operator(&mut p_state, &mut tokens);
                p_state.literal.push(ch);
            }
            _ => {
                end_both(&mut p_state, &mut tokens, location);
                p_state.errors.push(to_error!(
                    location,
                    "Character not supported by compiler: {ch}"
                ));
            }
        }
        location.incr_col();
    }
    end_both(&mut p_state, &mut tokens, location);
    Res::from((tokens, p_state.errors))
}

pub fn parse(expression: &str, location: &mut Location) -> Node {
    let tokens = get_tokens(expression, location);
    println!("\nTokens = {:?}", tokens.result);

    Node::Leaf(Literal::Const("1".into()))
}
