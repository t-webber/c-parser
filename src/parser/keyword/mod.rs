//! Module to handle keywords, convert them to operators and push them into the
//! [`Ast`].

extern crate alloc;
pub mod attributes;
pub mod control_flow;
pub mod functions;
pub mod sort;

use alloc::vec::IntoIter;

use control_flow::is_node_case_context;
use sort::{KeywordParsing, PushInNode as _};

use super::parse_content::parse_block;
use super::state::ParsingState;
use super::types::Ast;
use crate::Location;
use crate::errors::api::SingleRes;
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
) -> SingleRes<()> {
    let case_context = is_node_case_context(current);
    let parsed_keyword = KeywordParsing::from((keyword, case_context));
    parsed_keyword
        .push_in_node(current)
        .map_err(|msg| location.into_failure(msg))?;
    parse_block(tokens, p_state, current)
}
