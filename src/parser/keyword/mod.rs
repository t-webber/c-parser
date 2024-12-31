extern crate alloc;
pub mod attributes;
pub mod control_flow;
pub mod functions;
pub mod types;

use alloc::vec::IntoIter;

use control_flow::is_node_case_context;
use types::{KeywordParsing, PushInNode as _};

use super::parse_content::parse_block;
use super::state::ParsingState;
use super::tree::ast::Ast;
use crate::Location;
use crate::errors::api::CompileError;
use crate::lexer::api::{Keyword, Token};

#[allow(clippy::todo, reason = "not yet implemented")]
pub fn handle_keyword(
    keyword: Keyword,
    current: &mut Ast,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Result<(), CompileError> {
    let case_context = is_node_case_context(current);
    let parsed_keyword = KeywordParsing::from((keyword, case_context));
    parsed_keyword
        .push_in_node(current)
        .map_err(|msg| location.into_error(msg))?;
    parse_block(tokens, p_state, current)
}
