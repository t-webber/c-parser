extern crate alloc;
use alloc::vec::IntoIter;
use core::mem;

use super::state::{Block, ParsingState};
use super::tree::binary::BinaryOperator;
use super::tree::node::Node;
use super::tree::unary::UnaryOperator;
use crate::errors::compile::{as_error, to_error, CompileError};
use crate::errors::location::Location;
use crate::lexer::api::tokens_types::{Symbol, Token};
use crate::parser::parse_block;
use crate::parser::tree::{ListInitialiser, TernaryOperator};

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

fn handle_brace_open(current: &mut Node) -> Result<TodoState, String> {
    let res = current.can_push_list_initialiser();
    println!("BRACE OPEN = {res:?}");
    match res {
        Err(op) => Err(format!(
            "Found operator '{op}' applied on list initialiser '{{}}', but this is not allowed."
        )),
        Ok(true) => {
            current.push_block_as_leaf(Node::ListInitialiser(ListInitialiser::default()))?;
            Ok(TodoState::None)
        }
        Ok(false) => Ok(TodoState::OpenBraceBlock),
    }
}

// TODO: check for nested
enum TodoState {
    None,
    OpenBraceBlock,
    CloseBraceBlock,
    EndBlock,
    OpenParens,
    CloseParens,
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
        // braces & blocks
        BraceOpen => return handle_brace_open(current),
        BraceClose => {
            let res = current.edit_list_initialiser(&|_, full| *full = true);
            println!("BRACE CLOSE = {res:?}");
            if res.is_err() {
                return Ok(TodoState::CloseBraceBlock);
            }
        }
        BracketOpen => return Ok(TodoState::OpenBracket),
        BracketClose => return Ok(TodoState::CloseBracket),
        ParenthesisOpen => {
            if !current.try_make_function() {
                return Ok(TodoState::OpenParens);
            }
        }
        ParenthesisClose => {
            if !current.try_close_function() {
                return Ok(TodoState::CloseParens);
            }
        }
        SemiColon => return Ok(TodoState::EndBlock),
    }
    Ok(TodoState::None)
}

fn mismatched_err(open: char, close: char) -> String {
    format!(
        "Mismatched {open}: You either forgot a closing {close} or there is an extra semi-colon."
    )
}

pub fn handle_symbol(
    symbol: &Symbol,
    current: &mut Node,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Result<(), CompileError> {
    match handle_one_symbol(symbol, current, p_state).map_err(|err| to_error!(location, "{err}"))? {
        // semi-colon
        TodoState::EndBlock => Ok(()),
        // parenthesis
        TodoState::CloseParens => {
            p_state.opened_blocks.push(Block::Parenthesis);
            Ok(())
        }
        TodoState::OpenParens => {
            let mut parenthesized_block = Node::Empty;
            parse_block(tokens, p_state, &mut parenthesized_block)?;
            if p_state.opened_blocks.pop() == Some(Block::Parenthesis) {
                current
                    .push_block_as_leaf(Node::ParensBlock(Box::from(parenthesized_block)))
                    .map_err(|err| as_error!(location, "{err}"))?;
                parse_block(tokens, p_state, current)
            } else {
                Err(to_error!(location, "{}", mismatched_err('(', ')')))
            }
        }
        TodoState::None => parse_block(tokens, p_state, current),
        // bracket
        TodoState::CloseBracket => {
            p_state.opened_blocks.push(Block::Bracket);
            Ok(())
        }
        TodoState::OpenBracket => {
            let mut bracket_node = Node::Empty;
            parse_block(tokens, p_state, &mut bracket_node)?;
            if p_state.opened_blocks.pop() == Some(Block::Bracket) {
                current
                    .push_op(BOp::ArraySubscript)
                    .map_err(|err| to_error!(location, "{err}"))?;
                current
                    .push_block_as_leaf(bracket_node)
                    .map_err(|err| to_error!(location, "{err}"))?;
                parse_block(tokens, p_state, current)
            } else {
                Err(to_error!(location, "{}", mismatched_err('[', ']')))
            }
        }
        // brace
        TodoState::CloseBraceBlock => {
            p_state.opened_blocks.push(Block::Brace);
            Ok(())
        }
        TodoState::OpenBraceBlock => {
            let mut brace_block = Node::Empty;
            parse_block(tokens, p_state, &mut brace_block)?;
            if p_state.opened_blocks.pop() == Some(Block::Brace) {
                let old = mem::take(current);
                *current = Node::Block(vec![old, brace_block]);
                parse_block(tokens, p_state, current)
            } else {
                Err(to_error!(location, "{}", mismatched_err('{', '}')))
            }
        }
    }
}
