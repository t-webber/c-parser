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

impl Ast {
    /// Main handler to push a keyword into an [`Ast`].
    ///
    /// This function deals also the recursion calls.
    pub(super) fn push_keyword(
        &mut self,
        keyword: Keyword,
        p_state: &ParsingState,
        location: ErrorLocation,
    ) -> Res<ParseAction> {
        let ctx = if p_state.is_in_switch() {
            Context::Switch
        } else {
            Context::from(&*self)
        };
        let parsed_keyword = match KeywordParsing::try_from((keyword, ctx)) {
            Ok(ok) => ok,
            Err(msg) => return Res::from_err(location.into_crash(msg)),
        };
        let ast_push_ctx = match parsed_keyword {
            KeywordParsing::Attr(_) => AstPushContext::UserVariable,
            KeywordParsing::Pushable(PushableKeyword::Else) => AstPushContext::Else,
            KeywordParsing::CtrlFlow(_)
            | KeywordParsing::False
            | KeywordParsing::Func(_)
            | KeywordParsing::Null
            | KeywordParsing::True => AstPushContext::None,
        };
        if self.can_push_leaf_with_ctx(ast_push_ctx) {
            if let Err(msg) = parsed_keyword.push_in_node(self) {
                Res::from_err(location.into_crash(msg))
            } else {
                Res::ok(ParseAction::Continue)
            }
        } else if let AstValue::BracedBlock(BracedBlock { elts, full: false }) = &mut self.value {
            if let Some(last) = elts.last_mut() {
                if last.is_empty() {
                    return if let Err(msg) = parsed_keyword.push_in_node(last) {
                        Res::from_err(location.into_crash(msg))
                    } else {
                        Res::ok(ParseAction::Continue)
                    };
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
            if let Err(msg) = parsed_keyword.push_in_node(&mut new) {
                return Res::from_err(location.into_crash(msg));
            }
            elts.push(new);
            Res::ok(ParseAction::Continue)
        } else {
            unreachable!("trying to push {parsed_keyword:?} in {self}")
        }
    }
}
