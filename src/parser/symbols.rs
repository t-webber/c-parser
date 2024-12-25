extern crate alloc;
use alloc::vec::IntoIter;

#[allow(clippy::enum_glob_use)]
use Symbol::*;
use {BinaryOperator as BOp, UnaryOperator as UOp};

use super::blocks::blocks_handler;
use super::state::ParsingState;
use super::tree::binary::BinaryOperator;
use super::tree::node::Node;
use super::tree::unary::UnaryOperator;
use crate::errors::compile::CompileError;
use crate::errors::location::Location;
use crate::lexer::api::tokens_types::{Symbol, Token};
use crate::parser::blocks::BlockState;
use crate::parser::tree::TernaryOperator;

fn handle_comma(current: &mut Node) -> Result<(), String> {
    if current
        .edit_list_initialiser(&|vec, _| vec.push(Node::Empty))
        .is_err()
    {
        current.push_op(BinaryOperator::Comma)?;
    }
    Ok(())
}

fn handle_double_binary(
    current: &mut Node,
    bin_op: BinaryOperator,
    un_op: UnaryOperator,
) -> Result<(), String> {
    current
        .push_op(bin_op)
        .map_or_else(|_| current.push_op(un_op), |()| Ok(()))
}

fn handle_double_unary(
    current: &mut Node,
    first: UnaryOperator,
    second: UnaryOperator,
) -> Result<(), String> {
    current
        .push_op(first)
        .map_or_else(|_| current.push_op(second), |()| Ok(()))
}

#[allow(clippy::cognitive_complexity)]
fn handle_one_symbol(symbol: &Symbol, current: &mut Node) -> Result<BlockState, String> {
    match symbol {
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
        ShiftLeft => current.push_op(BOp::ShiftLeft)?,
        ShiftRight => current.push_op(BOp::ShiftRight)?,
        SubAssign => current.push_op(BOp::SubAssign)?,
        XorAssign => current.push_op(BOp::XorAssign)?,
        ShiftLeftAssign => current.push_op(BOp::ShiftLeftAssign)?,
        ShiftRightAssign => current.push_op(BOp::ShiftRightAssign)?,
        // unique non mirrors
        Arrow => current.push_op(BOp::StructEnumMemberPointerAccess)?,
        Dot => current.push_op(BOp::StructEnumMemberAccess)?,
        // comma
        Comma => handle_comma(current)?,
        // postfix has smaller precedence than prefix //TODO: make sure this works
        Increment => handle_double_unary(current, UOp::PostfixIncrement, UOp::PrefixIncrement)?,
        Decrement => handle_double_unary(current, UOp::PostfixDecrement, UOp::PrefixDecrement)?,
        // binary and unary operators //TODO: not sure this is good, may not work on extreme cases
        Ampercent => handle_double_binary(current, BOp::BitwiseAnd, UOp::AddressOf)?,
        Minus => handle_double_binary(current, BOp::Subtract, UOp::Minus)?,
        Plus => handle_double_binary(current, BOp::Add, UOp::Plus)?,
        Star => handle_double_binary(current, BOp::Multiply, UOp::Indirection)?,
        // ternary (only ternary because trigraphs are ignored, and colon is sorted in main function
        // in mod.rs)
        Interrogation => current.push_op(TernaryOperator)?,
        Colon => current.handle_colon()?,
        // braces & blocks
        SemiColon => return Ok(BlockState::SemiColon),
        BraceOpen => return Ok(BlockState::OpenBraceBlock),
        BraceClose => return Ok(BlockState::CloseBraceBlock),
        BracketOpen => return Ok(BlockState::OpenBracket),
        BracketClose => return Ok(BlockState::CloseBracket),
        ParenthesisOpen => return Ok(BlockState::OpenParens),
        ParenthesisClose => return Ok(BlockState::CloseParens),
    }
    Ok(BlockState::None)
}

pub fn handle_symbol(
    symbol: &Symbol,
    current: &mut Node,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Result<(), CompileError> {
    match handle_one_symbol(symbol, current) {
        Err(err) => Err(location.into_error(err)),
        Ok(block_state) => blocks_handler(current, tokens, p_state, location, &block_state),
    }
}
