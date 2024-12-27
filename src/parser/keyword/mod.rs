extern crate alloc;
mod types;

use alloc::vec::IntoIter;

use types::KeywordParsing;

use super::state::ParsingState;
use super::tree::node::Node;
use crate::errors::api::CompileError;
use crate::lexer::api::{Keyword, Token};
use crate::Location;

#[allow(clippy::todo, reason = "not yet implemented")]
pub fn handle_keyword(
    keyword: Keyword,
    _current: &mut Node,
    _p_state: &mut ParsingState,
    _tokens: &mut IntoIter<Token>,
    _location: Location,
) -> Result<(), CompileError> {
    let case_context = true; // node.is_in_case_context();
    match KeywordParsing::from((keyword, case_context)) {
        KeywordParsing::Nullptr => todo!(),
        KeywordParsing::False => todo!(),
        KeywordParsing::True => todo!(),
        KeywordParsing::CtrlFlow(_) => todo!(),
        KeywordParsing::Attr(_) => todo!(),
        KeywordParsing::Func(_) => todo!(),
    }
}
