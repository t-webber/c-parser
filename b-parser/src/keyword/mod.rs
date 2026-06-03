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
use crate::errors::{ErrorLocation, IntoError as _, Res};
use lexer::Keyword;
use crate::symbols::BracedBlock;
use crate::tree::Ast;
use crate::tree::AstPushContext;

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
    } else if let Ast::BracedBlock(BracedBlock { elts, full: false }) = current {
        match elts.last_mut() {
            Some(last) if last.is_empty() => {
                parsed_keyword
                    .push_in_node(last)
                    .map_err(|msg| location.to_crash(msg))?;
            }
            Some(Ast::BracedBlock(_) | Ast::ControlFlow(_)) | None => {
                let mut new = Ast::Empty;
                parsed_keyword
                    .push_in_node(&mut new)
                    .map_err(|msg| location.into_crash(msg))?;
                elts.push(new);
            }
            Some(_) => {
                return location
                    .into_crash(
                        "Invalid keyword in current context. Perhaps a missing ';'".to_owned(),
                    )
                    .into_res();
            }
        }
    } else {
        unreachable!("trying to push {parsed_keyword:?} in {current}")
    }
    Res::ok(ParseAction::Continue)
}
