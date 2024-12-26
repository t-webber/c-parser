use crate::parser::tree::binary::BinaryOperator;
use crate::parser::tree::node::Node;
use crate::parser::tree::unary::UnaryOperator;

pub fn handle_comma(current: &mut Node) -> Result<(), String> {
    if current
        .edit_list_initialiser(&|vec, _| vec.push(Node::Empty))
        .is_err()
    {
        current.push_op(BinaryOperator::Comma)?;
    }
    Ok(())
}

pub fn handle_double_binary(
    current: &mut Node,
    bin_op: BinaryOperator,
    un_op: UnaryOperator,
) -> Result<(), String> {
    current
        .push_op(bin_op)
        .map_or_else(|_| current.push_op(un_op), |()| Ok(()))
}

pub fn handle_double_unary(
    current: &mut Node,
    first: UnaryOperator,
    second: UnaryOperator,
) -> Result<(), String> {
    current
        .push_op(first)
        .map_or_else(|_| current.push_op(second), |()| Ok(()))
}
