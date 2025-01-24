//! Module to handle keywords, convert them to operators and push them into the
//! [`Ast`].

extern crate alloc;
pub mod attributes;
pub mod control_flow;
pub mod functions;
pub mod sort;

use alloc::vec::IntoIter;

use control_flow::pushable::PushableKeyword;
use sort::{Context, KeywordParsing, PushInNode as _};

use super::modifiers::ast::can_push::{AstPushContext, CanPush as _};
use super::parse_content::parse_block;
use super::state::ParsingState;
use super::types::Ast;
use super::types::braced_blocks::BracedBlock;
use crate::Location;
use crate::errors::api::Res;
use crate::lexer::api::{Keyword, Token};

/// Main handler to push a keyword into an [`Ast`].
///
/// This function deals also the recursion calls.
pub fn handle_keyword(
    keyword: Keyword,
    current: &mut Ast,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Res<()> {
    let ctx = if p_state.is_in_switch() {
        Context::Switch
    } else {
        Context::from(&*current)
    };
    let parsed_keyword: KeywordParsing =
        KeywordParsing::try_from((keyword, ctx)).map_err(|msg| location.to_crash(msg))?;
    let ast_push_ctx = match parsed_keyword {
        KeywordParsing::Attr(_) => AstPushContext::UserVariable,
        KeywordParsing::Pushable(PushableKeyword::Else) => AstPushContext::Else,
        KeywordParsing::CtrlFlow(_)
        | KeywordParsing::False
        | KeywordParsing::Func(_)
        | KeywordParsing::Nullptr
        | KeywordParsing::True => AstPushContext::None,
    };
    if current.can_push_leaf_with_ctx(ast_push_ctx) {
        parsed_keyword
            .push_in_node(current)
            .map_err(|msg| location.into_crash(msg))?;
    } else if let Ast::BracedBlock(BracedBlock { elts, full: false }) = current {
        elts.push(Ast::Empty);
        parsed_keyword
            .push_in_node(elts.last_mut().expect("just pushed"))
            .map_err(|msg| location.into_crash(msg))?;
    } else {
        panic!("trying to push {parsed_keyword:?} in {current}")
    }
    parse_block(tokens, p_state, current)
}
