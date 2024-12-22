use super::state::ParsingState;
use super::tree::binary::BinaryOperator;
use super::tree::node::Node;
use super::tree::unary::UnaryOperator;
use crate::errors::compile::CompileError;
use crate::errors::location::Location;
use crate::lexer::api::tokens_types::{Symbol, Token};
use crate::parser::parse_block;
use crate::parser::tree::TernaryOperator;
use crate::{as_error, to_error};
extern crate alloc;
use alloc::vec::IntoIter;

fn safe_decr(counter: &mut usize, ch: char) -> Result<ParensEvolution, String> {
    *counter = counter
        .checked_sub(1)
        .ok_or_else(|| format!("Mismactched closing '{ch}'"))?;
    Ok(ParensEvolution::Close)
}

fn handle_double(
    current: &mut Node,
    bin_op: BinaryOperator,
    un_op: UnaryOperator,
) -> Result<(), String> {
    current
        .push_op(bin_op)
        .map_or_else(|_| current.push_op(un_op), |()| Ok(()))
}

enum ParensEvolution {
    Open,
    Close, // TODO: [ and ( is different !
    // TODO: must store successive and nested
    None,
    SemiColon,
}
use BinaryOperator as BOp;
use UnaryOperator as UOp;

fn handle_one_symbol(
    symbol: &Symbol,
    current: &mut Node,
    p_state: &mut ParsingState,
) -> Result<ParensEvolution, String> {
    use ParensEvolution as PE;
    #[allow(clippy::enum_glob_use)]
    use Symbol::*;
    match symbol {
        Colon if p_state.wanting_colon => (),
        _ if p_state.wanting_colon => {
            return Err(format!("Wanting colon after 'goto', but found {symbol:?}"))
        }
        // mirror unary
        BitwiseNot => current.push_op(UOp::BitwiseNot)?,
        LogicalNot => current.push_op(UOp::LogicalNot)?,
        // mirror binary
        Assign => current.push_op(BOp::Assign)?,
        BitwiseOr => current.push_op(BOp::BitwiseOr)?,
        BitwiseXor => current.push_op(BOp::BitwiseXor)?,
        Divide => current.push_op(BOp::Divide)?,
        Gt => current.push_op(BOp::Gt)?,
        Lt => current.push_op(BOp::Lt)?,
        Modulo => current.push_op(BOp::Modulo)?,
        AddAssign => current.push_op(BOp::AddAssign)?,
        AndAssign => current.push_op(BOp::AndAssign)?,
        Different => current.push_op(BOp::Different)?,
        DivAssign => current.push_op(BOp::DivAssign)?,
        Equal => current.push_op(BOp::Equal)?,
        Ge => current.push_op(BOp::Ge)?,
        Le => current.push_op(BOp::Le)?,
        LogicalAnd => current.push_op(BOp::LogicalAnd)?,
        LogicalOr => current.push_op(BOp::LogicalOr)?,
        ModAssign => current.push_op(BOp::ModAssign)?,
        MulAssign => current.push_op(BOp::MulAssign)?,
        OrAssign => current.push_op(BOp::OrAssign)?,
        LeftShift => current.push_op(BOp::LeftShift)?,
        RightShift => current.push_op(BOp::RightShift)?,
        SubAssign => current.push_op(BOp::SubAssign)?,
        XorAssign => current.push_op(BOp::XorAssign)?,
        LeftShiftAssign => current.push_op(BOp::LeftShiftAssign)?,
        RightShiftAssign => current.push_op(BOp::RightShiftAssign)?,
        // unique non mirrors
        Arrow => current.push_op(BOp::StructEnumMemberPointerAccess)?,
        Dot => current.push_op(BinaryOperator::StructEnumMemberAccess)?,
        // postfix has smaller precedence than prefix
        Increment => current
            .push_op(UOp::PostfixIncrement)
            .unwrap_or(current.push_op(UOp::PrefixIncrement)?), // Operator is left to right, so, if an error occurs, current isn't modified
        Decrement => current
            .push_op(UOp::PostfixDecrement)
            .unwrap_or(current.push_op(UOp::PrefixDecrement)?), // Operator is left to right, so, if an error occurs, current isn't modified
        // binary and unary operators //TODO: not sure this is good, may not work on extreme cases
        Ampercent => handle_double(current, BOp::BitwiseAnd, UOp::AddressOf)?,
        Minus => handle_double(current, BOp::Subtract, UOp::Minus)?,
        Plus => handle_double(current, BOp::Add, UOp::Plus)?,
        Star => handle_double(current, BOp::Multiply, UOp::Indirection)?,
        // ternary (only ternary because trigraphs are ignored, and colon is sorted in main function in mod.rs)
        Interrogation => {
            p_state.ternary += 1;
            current.push_op(TernaryOperator)?;
        }

        Colon => {
            if p_state.ternary == 0 {
                return Err("Operator mismatch: found unmatched ':' character. Missing 'goto' keyword or '?' symbol.".into());
            }
            current.handle_colon()?;
            p_state.ternary -= 1;
        }
        //
        SemiColon => return Ok(PE::SemiColon),
        Comma => todo!(),
        // parenthesis
        BraceOpen => {
            p_state.braces += 1;
            return Ok(PE::Open);
        }
        BraceClose => return safe_decr(&mut p_state.braces, '}'),
        BracketOpen => {
            p_state.brackets += 1;
            return Ok(PE::Open);
        }
        BracketClose => return safe_decr(&mut p_state.brackets, ']'),
        ParenthesisOpen => {
            p_state.parenthesis += 1;
            return Ok(PE::Open);
        }
        ParenthesisClose => return safe_decr(&mut p_state.parenthesis, ')'),
    }
    Ok(PE::None)
}

pub fn handle_symbol(
    symbol: &Symbol,
    current: &mut Node,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Result<(), CompileError> {
    match handle_one_symbol(symbol, current, p_state).map_err(|err| to_error!(location, "{err}"))? {
        ParensEvolution::Open => {
            let mut parenthesized_block = Node::Empty;
            parse_block(tokens, p_state, &mut parenthesized_block)?;
            current
                .push_block_as_leaf(parenthesized_block)
                .map_err(|err| as_error!(location, "{err}"))?;
            parse_block(tokens, p_state, current)
        }
        ParensEvolution::None => parse_block(tokens, p_state, current),
        ParensEvolution::Close | ParensEvolution::SemiColon => Ok(()),
    }
}
