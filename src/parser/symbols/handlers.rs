//! Handlers to be called when a symbol can represent by multiple operator.

use super::blocks::braced_blocks::BracedBlock;
use super::blocks::default::ListInitialiser;
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::modifiers::list_initialiser::apply_to_last_list_initialiser;
use crate::parser::modifiers::make_lhs::try_apply_comma_to_variable;
use crate::parser::modifiers::push::Push as _;
use crate::parser::operators::api::{Binary, BinaryOperator, Ternary, Unary, UnaryOperator};
use crate::parser::tree::api::Ast;

impl Ast {
    /// Handler to push a symbol that can be represented by a binary and a unary
    /// operator.
    pub fn handle_binary_unary(
        &mut self,
        bin_op: BinaryOperator,
        un_op: UnaryOperator,
    ) -> Result<(), String> {
        self.push_op(bin_op)
            .map_or_else(|_| self.push_op(un_op), |()| Ok(()))
    }

    /// Adds the colon of a
    /// [`TernaryOperator`](crate::parser::operators::api::TernaryOperator).
    ///
    /// This method finds a ternary operator, and changes its reading state to
    /// failure.
    pub fn handle_colon(&mut self) -> Result<(), String> {
        #[cfg(feature = "debug")]
        crate::lgp!("Pushing colon in {self}");
        match self {
            Self::Ternary(Ternary { failure: failure @ None, .. }) => {
                *failure = Some(Self::empty_box());
                Ok(())
            }
            // label
            Self::Variable(var) => {
                if let Some(new) = var.push_colon()? {
                    *self = new;
                }
                Ok(())
            }
            Self::Empty
            | Self::Leaf { .. }
            | Self::ParensBlock(_)
            | Self::FunctionCall(_)
            | Self::ListInitialiser(ListInitialiser { full: true, .. })
            | Self::BracedBlock(BracedBlock { full: true, .. }) =>
                Err("Ternary symbol mismatched: found a ':' symbol without '?'.".to_owned()),
            Self::Unary(Unary { arg, .. })
            | Self::Binary(Binary { arg_r: arg, .. })
            | Self::Ternary(Ternary { failure: Some(arg), .. }) => arg.handle_colon(),
            Self::ListInitialiser(ListInitialiser { full: false, elts: vec })
            | Self::BracedBlock(BracedBlock { elts: vec, full: false })
            | Self::FunctionArgsBuild(vec) =>
                vec.last_mut().expect("Created with one elt").handle_colon(),
            Self::ControlFlow(ctrl) =>
                if ctrl.push_colon() {
                    Ok(())
                } else {
                    Err(
                    "Found extra ':': Tried to push colon in a control flow that wasn't expecting one.".to_owned(),
                )
                },
            Self::Cast(cast) =>
                if cast.full {
                    Err("Found extra ':': colon is illegal for cast.".to_owned())
                } else {
                    cast.value.handle_colon()
                },
        }
    }

    /// Handler to push a comma into an [`Self`]
    pub fn handle_comma(&mut self) -> Result<(), String> {
        if let Self::FunctionArgsBuild(vec) = self {
            vec.push(Self::Empty);
        } else if apply_to_last_list_initialiser(self, &|vec, _| vec.push(Self::Empty)).is_none()
            && !try_apply_comma_to_variable(self)?
        {
            self.push_op(BinaryOperator::Comma)?;
        }
        Ok(())
    }

    /// Handler to push a symbol that can be represented by 2 different unary
    /// operators.
    pub fn handle_double_unary(
        &mut self,
        first: UnaryOperator,
        second: UnaryOperator,
    ) -> Result<(), String> {
        self.push_op(first)
            .map_or_else(|_| self.push_op(second), |()| Ok(()))
    }
}
