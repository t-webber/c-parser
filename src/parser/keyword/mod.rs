extern crate alloc;
pub mod types;

use alloc::vec::IntoIter;

use types::controlflow::ControlFlowKeyword;
use types::{KeywordParsing, PushInNode as _};

use super::parse_content::parse_block;
use super::state::ParsingState;
use super::tree::node::Ast;
use crate::errors::api::CompileError;
use crate::lexer::api::{Keyword, Token};
use crate::Location;

#[allow(clippy::todo, reason = "not yet implemented")]
pub fn handle_keyword(
    keyword: Keyword,
    current: &mut Ast,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Result<(), CompileError> {
    let case_context = ControlFlowKeyword::is_in_case_context(current);
    let parsed_keyword = KeywordParsing::from((keyword, case_context));
    parsed_keyword
        .push_in_node(current)
        .map_err(|msg| location.into_error(msg))?;
    parse_block(tokens, p_state, current)
}
