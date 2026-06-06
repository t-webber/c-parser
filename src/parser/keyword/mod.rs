//! Module to handle keywords, convert them to operators and push them into the
//! [`Ast`].

pub mod attributes;
pub mod control_flow;
pub mod functions;
pub mod sort;

use control_flow::pushable::PushableKeyword;
use sort::{Context, KeywordParsing, PushInNode as _};

use super::parse_content::ParseAction;
use super::state::ParsingState;
use crate::errors::api::{ErrorLocation, IntoError as _, Res};
use crate::lexer::api::Keyword;
use crate::parser::api::AstValue;
use crate::parser::symbols::api::BracedBlock;
use crate::parser::tree::Ast;
use crate::parser::tree::api::AstPushContext;

/// Main handler to push a keyword into an [`Ast`].
///
/// This function deals also the recursion calls.
pub fn handle_keyword(
    keyword: Keyword,
    current: &mut Ast,
    p_state: &ParsingState,
    location: ErrorLocation,
) -> Res<ParseAction> {
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
        | KeywordParsing::Null
        | KeywordParsing::True => AstPushContext::None,
    };
    if current.can_push_leaf_with_ctx(ast_push_ctx) {
        parsed_keyword
            .push_in_node(current)
            .map_err(|msg| location.into_crash(msg))?;
        Res::ok(ParseAction::Continue)
    } else if let AstValue::BracedBlock(BracedBlock { elts, full: false }) = &mut current.value {
        if let Some(last) = elts.last_mut() {
            if last.is_empty() {
                parsed_keyword
                    .push_in_node(last)
                    .map_err(|msg| location.to_crash(msg))?;
                return Res::ok(ParseAction::Continue);
            }
            if !matches!(last.value, AstValue::BracedBlock(_) | AstValue::ControlFlow(_)) {
                return location
                    .into_crash(
                        "Invalid keyword in current context. Perhaps a missing ';'".to_owned(),
                    )
                    .into_res();
            }
        }
        let mut new = AstValue::Empty.into();
        parsed_keyword
            .push_in_node(&mut new)
            .map_err(|msg| location.into_crash(msg))?;
        elts.push(new);
        Res::ok(ParseAction::Continue)
    } else {
        unreachable!("trying to push {parsed_keyword:?} in {current}")
    }
}
