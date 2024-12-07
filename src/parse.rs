#![allow(dead_code)]

use crate::tree::{Literal, Node};
use core::mem::take;

#[derive(Debug)]
enum Symbol {
    Plus,
    Minus,
    ParenthesisOpen,
    ParenthesisClose,
    BracketOpen,
    BracketClose,
    Dot,
    BraceOpen,
    BraceClose,
    BitwiseNot,
    LogicalNot,
    Star,
    Ampercent,
    Modulo,
    Divide,
    Gt,
    Lt,
    Assign,
    BitwiseOr,
    BitwiseXor,
    Comma,
    Interrogation,
    Colon,
    //
    Arrow,
    Increment,
    Decrement,
    Le,
    ShiftLeft,
    ShiftRight,
    Ge,
    Equal,
    Different,
    LogicialAnd,
    LogicalOr,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    //
    ShiftLeftAssign,
    ShiftRightAssign,
}

#[derive(Debug)]
enum Token {
    Symbol(Symbol),
    Literal(String),
    Char(char),
    Str(String),
}

#[derive(Default, Debug)]
struct State {
    first: char,
    second: char,
    third: char,
    literal: String,
    single_quote: bool,
    double_quote: bool,
    escape: bool,
}

const NULL: char = '\0';

impl State {
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
            panic!("This is not meant to happen. Called try_operator on none empty self, and no operator was returned. State: {self:?}");
        }
        op
    }

    fn is_empty(&self) -> bool {
        self.first == NULL && self.second == NULL && self.third == NULL
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
            ('&', '&', _) => (2, Some(OT::LogicialAnd)),
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
                "This is not meant to happen. Some unsupported symbols were found in the operator part of the state. State: {self:?}"
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

fn end_both(state: &mut State, tokens: &mut Vec<Token>) {
    end_operator(state, tokens);
    end_literal(state, tokens);
}

fn end_literal(state: &mut State, tokens: &mut Vec<Token>) {
    if !state.literal.is_empty() {
        tokens.push(Token::Literal(take(&mut state.literal)));
    }
    assert!(state.literal.is_empty(), "Not possible: The string was just cleared, except if i am stupid and take doesn't clear ??!! State:{:?}", &state);
}

fn end_operator(state: &mut State, tokens: &mut Vec<Token>) {
    let mut idx: usize = 0;
    while !state.is_empty() && idx <= 2 {
        idx += 1;
        if let Some(operator) = state.try_to_operator() {
            tokens.push(Token::Symbol(operator));
        } else {
            panic!(
                "This can't happen, as state is not empty! State: {:?}",
                &state
            );
        }
    }
    assert!(state.is_empty(), "Not possible: executing 3 times the conversion, with stritcly decreasing number of non empty elements! This can't happen. State: {:?}", &state);
}

fn end_string(state: &mut State, tokens: &mut Vec<Token>) {
    if !state.literal.is_empty() {
        tokens.push(Token::Str(take(&mut state.literal)));
    }
    assert!(state.literal.is_empty(), "Not possible: The string was just cleared, except if i am stupid and take doesn't clear ??!! State:{:?}", &state);
}

fn get_tokens(expression: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut state = State::default();
    for ch in expression.chars() {
        println!("State = {:?}\tTokens = {:?}", &state, &tokens);
        match ch {
            // Deal with static strings and chars
            '\'' if !state.escape && state.single_quote => state.single_quote = false,
            _ if state.single_quote => tokens.push(Token::Char(ch)),
            '\"' if !state.escape && state.double_quote => {
                state.double_quote = false;
                end_string(&mut state, &mut tokens);
            }
            _ if state.double_quote => state.literal.push(ch),
            '\'' => {
                end_both(&mut state, &mut tokens);
                state.single_quote = true;
            }
            '"' => {
                end_both(&mut state, &mut tokens);
                state.double_quote = true;
            }

            // Operator symbols
            '+' | '-' | '(' | ')' | '[' | ']' | '.' | '{' | '}' | '~' | '!' | '*' | '&' | '%'
            | '/' | '>' | '<' | '=' | '|' | '^' | ',' | '?' | ':' => {
                end_literal(&mut state, &mut tokens);
                if let Some(operator) = state.push(ch) {
                    tokens.push(Token::Symbol(operator));
                }
            }

            // Whitespace: end of everyone
            _ if ch.is_whitespace() => {
                end_both(&mut state, &mut tokens);
            }

            // Whitespace: end of everyone
            _ => {
                end_operator(&mut state, &mut tokens);
                state.literal.push(ch);
            }
        }
    }
    end_both(&mut state, &mut tokens);
    tokens
}

pub fn parse(expression: &str) -> Node {
    let tokens = get_tokens(expression);
    println!("\nTokens = {:?}", tokens);

    Node::Leaf(Literal::Const("1".into()))
}
