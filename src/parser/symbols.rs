use super::state::ParsingState;
use super::tree::binary::BinaryOperator;
use super::tree::node::Node;
use super::tree::unary::UnaryOperator;
use crate::errors::compile::{as_error, to_error, CompileError};
use crate::errors::location::Location;
use crate::lexer::api::tokens_types::{Symbol, Token};
use crate::parser::parse_block;
use crate::parser::tree::{ListInitialiser, TernaryOperator};
extern crate alloc;
use alloc::vec::IntoIter;

fn safe_decr(counter: &mut usize, ch: char) -> Result<TodoState, String> {
    *counter = counter
        .checked_sub(1)
        .ok_or_else(|| format!("Mismactched closing '{ch}'"))?;
    Ok(TodoState::CloseBlock)
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

// TODO: check for nested
enum TodoState {
    None,
    OpenParens,
    CloseBlock,
    OpenBracket,
    CloseBracket,
}

use {BinaryOperator as BOp, UnaryOperator as UOp};

fn handle_one_symbol(
    symbol: &Symbol,
    current: &mut Node,
    p_state: &ParsingState,
) -> Result<TodoState, String> {
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
        Dot => current.push_op(BOp::StructEnumMemberAccess)?,
        Comma => {
            if current
                .edit_list_initialiser(&|vec, _| vec.push(Node::Empty))
                .is_err()
            {
                current.push_op(BOp::Comma)?;
            }
        }
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

        BraceOpen if *current == Node::Empty => *current = Node::Block(vec![]),
        // braces & blocks
        BraceOpen => {
            if let Some(op) = current.contains_operators_on_right() {
                return Err(format!(
                    "Found operator '{op}' applied on list initialiser '{{}}', but this is not allowed."
                ));
            }
            current.push_block_as_leaf(Node::ListInitialiser(ListInitialiser::default()))?;
        }
        BraceClose => {
            if current
                .edit_list_initialiser(&|_, full| *full = true)
                .is_err()
            {
                return Err("Mismatched '}'. Found closing brace for a list initialiser, but list was not found.".into());
            }
        }
        BracketOpen => return Ok(TodoState::OpenBracket),
        BracketClose => return Ok(TodoState::CloseBracket),
        ParenthesisOpen => return Ok(TodoState::OpenParens),
        SemiColon | ParenthesisClose => return Ok(TodoState::CloseBlock),
    }
    Ok(TodoState::None)
}

pub fn handle_symbol(
    symbol: &Symbol,
    current: &mut Node,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Result<(), CompileError> {
    // TODO: i can't believe this works
    match handle_one_symbol(symbol, current, p_state).map_err(|err| to_error!(location, "{err}"))? {
        TodoState::OpenParens => {
            let mut parenthesized_block = Node::Empty;
            parse_block(tokens, p_state, &mut parenthesized_block)?;
            current
                .push_block_as_leaf(Node::ParensBlock(Box::from(parenthesized_block)))
                .map_err(|err| as_error!(location, "{err}"))?;
            parse_block(tokens, p_state, current)
        }
        TodoState::None => parse_block(tokens, p_state, current),
        TodoState::CloseBlock => Ok(()),
        TodoState::OpenBracket => {
            let mut bracket_node = Node::Empty;
            parse_block(tokens, p_state, &mut bracket_node)?;
            if p_state.closing_bracket {
                current
                    .push_op(BOp::ArraySubscript)
                    .map_err(|err| to_error!(location, "{err}"))?;
                current
                    .push_block_as_leaf(bracket_node)
                    .map_err(|err| to_error!(location, "{err}"))?;
                parse_block(tokens, p_state, current)
            } else {
                Err(to_error!(
                    location,
                    "Expected expression found block, as argument of an array subscript."
                ))
            }
        }
        TodoState::CloseBracket => {
            p_state.closing_bracket = true;
            Ok(())
        }
    }
}
