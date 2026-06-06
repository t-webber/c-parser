//! Module to convert a list of [`Token`] into an [`Ast`].

extern crate alloc;
use alloc::vec::IntoIter;

use super::literal::Literal;
use super::modifiers::push::Push as _;
use super::state::ParsingState;
use super::symbols::api::BracedBlock;
use super::symbols::handle_symbol;
use super::tree::api::Ast;
use super::variable::Variable;
use crate::errors::api::{IntoError as _, Res};
use crate::lexer::api::{Token, TokenValue};
use crate::parser::api::AstValue;

/// Indicates whether the current block should continue parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseAction {
    /// Continue parsing the current block.
    Continue,
    /// Stop parsing the current block (a closing delimiter was found).
    Stop,
}

impl Ast {
    /// Pushes a [`Literal`] into the [`Ast`]
    fn push_literal(&mut self, lit: Self) -> Res<ParseAction> {
        let location = lit.location.clone().expect("todo");
        if let Err(err) = self.push_block_as_leaf(lit) {
            Res::from_err(location.into_crash(err))
        } else {
            Res::ok(ParseAction::Continue)
        }
    }
}

/// Deletes unnecessary outer block if necessary
fn clean_nodes(nodes: Vec<Ast>) -> Ast {
    let mut cleaned = nodes
        .into_iter()
        .filter(|node| !node.is_empty())
        .collect::<Vec<_>>();
    if cleaned.len() == 1 {
        cleaned.pop().expect("len == 1")
    } else {
        AstValue::BracedBlock(BracedBlock { elts: cleaned, full: false }).into()
    }
}

/// Function to parse one node, and by recursivity, one block. At the end of the
/// block, this function stops and is recalled from [`parse`].
pub fn parse_block(
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    current: &mut Ast,
) -> Res<()> {
    let mut errors = vec![];
    loop {
        if let Some(token) = tokens.next() {
            #[cfg(feature = "debug")]
            println!("\n\x1b[36m{:20} on {current}\x1b[0m", format!("{token}"),);
            let (value, location) = token.into_value_location();
            let res = match value {
                TokenValue::Char(ch) =>
                    current.push_literal(AstValue::Leaf(Literal::Char(ch)).with_location(location)),
                TokenValue::Ident(val) => current
                    .push_literal(AstValue::Variable(Variable::from(val)).with_location(location)),
                TokenValue::Number(nb) => current
                    .push_literal(AstValue::Leaf(Literal::Number(nb)).with_location(location)),
                TokenValue::Str(val) =>
                    current.push_literal(AstValue::Leaf(Literal::Str(val)).with_location(location)),
                TokenValue::Symbol(symbol) =>
                    handle_symbol(symbol, current, p_state, tokens, location),
                TokenValue::Keyword(keyword) => current.push_keyword(keyword, p_state, location),
            };
            let has_failures = res.has_failures();
            let action = res.store_errors(&mut |err| errors.push(err));
            if !has_failures && action == Some(ParseAction::Continue) {
                continue;
            }
        }
        return Res::from(((), errors));
    }
}

/// Parses a list of tokens into an Abstract Syntax Tree.
///
/// This function manages the blocks with successive calls and checks.
#[must_use]
pub fn parse(tokens: Vec<Token>) -> Res<Ast> {
    let mut nodes = vec![];
    let mut errors = vec![];
    let mut tokens_iter = tokens.into_iter();
    while tokens_iter.len() != 0 {
        let mut outer_node_block = AstValue::BracedBlock(BracedBlock::default()).into();
        let mut p_state = ParsingState::default();
        let res = parse_block(&mut tokens_iter, &mut p_state, &mut outer_node_block);
        if res.has_failures() {
            errors.extend(res.into_errors());
            return Res::from((clean_nodes(nodes), errors));
        }
        errors.extend(res.into_errors());
        if p_state.has_opening_blocks() {
            errors.extend(p_state.mismatched_error());
            return Res::from((clean_nodes(nodes), errors));
        }
        nodes.push(outer_node_block);
    }
    Res::from((clean_nodes(nodes), errors))
}
