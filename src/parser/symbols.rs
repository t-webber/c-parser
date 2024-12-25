extern crate alloc;
use alloc::vec::IntoIter;
use core::mem;

use super::state::{Block, ParsingState};
use super::tree::binary::BinaryOperator;
use super::tree::node::Node;
use super::tree::unary::UnaryOperator;
use crate::errors::compile::{as_error, CompileError};
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

// TODO: check for nested
enum BlockState {
    None,
    OpenBraceBlock,
    CloseBraceBlock,
    SemiColon,
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
) -> Result<BlockState, String> {
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
        BraceOpen => return Ok(BlockState::OpenBraceBlock),
        BraceClose => return Ok(BlockState::CloseBraceBlock),
        BracketOpen => return Ok(BlockState::OpenBracket),
        BracketClose => return Ok(BlockState::CloseBracket),
        ParenthesisOpen => return Ok(BlockState::OpenParens),
        ParenthesisClose => return Ok(BlockState::CloseParens),
        SemiColon => return Ok(BlockState::SemiColon),
    }
    Ok(BlockState::None)
}

fn mismatched_err(open: char, close: char) -> String {
    format!(
        "Mismatched {open}: You either forgot a closing {close} or there is an extra semi-colon."
    )
}

fn blocks_handler(
    current: &mut Node,
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    location: Location,
    block_state: &BlockState,
) -> Result<(), CompileError> {
    match block_state {
        // semi-colon
        BlockState::SemiColon => {
            if let Node::Block(block) = current
                && !block.last().is_none_or(|node| *node == Node::Empty)
                {
                block.push(Node::Empty);
            } else if *current != Node::Empty {
                *current = Node::Block(vec![mem::take(current), Node::Empty]);
            } else {
                /* last is empty: nothing to be done */
            }
            parse_block(tokens, p_state, current)
        }
        // parenthesis
        BlockState::CloseParens if !current.try_close_function() => {
            p_state.opened_blocks.push(Block::Parenthesis);
            Ok(())
        }
        BlockState::OpenParens if !current.try_make_function() => {
            let mut parenthesized_block = Node::Empty;
            parse_block(tokens, p_state, &mut parenthesized_block)?;
            if p_state.opened_blocks.pop() == Some(Block::Parenthesis) {
                current
                    .push_block_as_leaf(Node::ParensBlock(Box::from(parenthesized_block)))
                    .map_err(|err| as_error!(location, "{err}"))?;
                parse_block(tokens, p_state, current)
            } else {
                Err(as_error!(location, "{}", mismatched_err('(', ')')))
            }
        }
        // bracket
        BlockState::CloseBracket => {
            p_state.opened_blocks.push(Block::Bracket);
            Ok(())
        }
        BlockState::OpenBracket => {
            let mut bracket_node = Node::Empty;
            parse_block(tokens, p_state, &mut bracket_node)?;
            if p_state.opened_blocks.pop() == Some(Block::Bracket) {
                if let Err(err) = current.push_op(BOp::ArraySubscript) { Err(as_error!(location, "{err}")) } else {
                    current
                    .push_block_as_leaf(bracket_node)
                    .map_err(|err| as_error!(location, "{err}"))?;
                    parse_block(tokens, p_state, current)
                }
            } else {
                Err(as_error!(location, "{}", mismatched_err('[', ']')))
            }
        }
        // brace
        BlockState::CloseBraceBlock
            if current
                .edit_list_initialiser(&|_, full| *full = true)
                .is_err() =>
        {
            p_state.opened_blocks.push(Block::Brace);
            Ok(())
        }
        BlockState::OpenBraceBlock => {
            match current.can_push_list_initialiser() {
                Err(op) => Err(as_error!(location, "Found operator '{op}' applied on list initialiser '{{}}', but this is not allowed.")),
                Ok(true) => {
                    current.push_block_as_leaf(Node::ListInitialiser(ListInitialiser::default())).map_err(|err| as_error!(location, "{err}"))?;
                    parse_block(tokens, p_state, current)
                }
                Ok(false) => {
                    let mut brace_block = Node::Empty;
                    parse_block(tokens, p_state, &mut brace_block)?;
                    if p_state.opened_blocks.pop() == Some(Block::Brace) {
                        if let Node::Block(vec) = current {
                            vec.push(brace_block);
                        } else if *current == Node::Empty {
                            *current = brace_block;
                        } else {
                            *current =
                                Node::Block(vec![mem::take(current), brace_block]);
                        }
                        parse_block(tokens, p_state, current)
                    } else {
                        Err(as_error!(location, "{}", mismatched_err('{', '}')))
                    }

                }
            }
        }
        // others
        BlockState::None
        | BlockState::OpenParens
        | BlockState::CloseParens
        | BlockState::CloseBraceBlock => parse_block(tokens, p_state, current),
    }
}

pub fn handle_symbol(
    symbol: &Symbol,
    current: &mut Node,
    p_state: &mut ParsingState,
    tokens: &mut IntoIter<Token>,
    location: Location,
) -> Result<(), CompileError> {
    match handle_one_symbol(symbol, current, p_state) {
        Err(err) => Err(as_error!(location, "{err}")),
        Ok(block_state) => blocks_handler(current, tokens, p_state, location, &block_state),
    }
}
