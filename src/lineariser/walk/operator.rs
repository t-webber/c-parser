//! Walks a ternary expression, updating state and creating symbols and basic
//! blocks.

use crate::errors::api::IntoError as _;
use crate::lineariser::basic_block::{BasicBlocks, Id};
use crate::lineariser::macros::attr;
use crate::lineariser::state::LState;
use crate::lineariser::symbol::Value;
use crate::parser::api::{Binary, Ternary};

impl Binary {
    /// Pushes some content into the [`BasicBlocks`].
    pub fn push_in(self, bbs: &mut BasicBlocks, state: &mut LState) -> Id {
        let Self { arg_l, arg_r, op } = self;
        let loc_l = arg_l.location();
        if arg_r.is_empty() {
            state.push_error(
                loc_l
                    .into_extended(op.as_location())
                    .fail("Missing RHS of binary operator".to_owned()),
            );
            return Id::NotFound;
        }
        let loc_r = arg_r.location();
        match (arg_l.push_in(bbs, state), arg_r.push_in(bbs, state)) {
            (Some(Id::NotFound), _) | (_, Some(Id::NotFound)) => Id::NotFound,
            res @ ((None, _) | (_, None)) => {
                if res.0.is_none() {
                    state.stat_not_expr(loc_l);
                }
                if res.1.is_none() {
                    state.stat_not_expr(loc_r);
                }
                Id::NotFound
            }
            (Some(Id::Found(left)), Some(Id::Found(right))) => state
                .push_element(
                    Value::Binary(op.drop_location(), left, right),
                    vec![attr!(BasicDataType Void)],
                )
                .into(),
        }
    }
}

impl Ternary {
    /// Pushes some content into the [`BasicBlocks`].
    pub fn push_in(self, bbs: &mut BasicBlocks, state: &mut LState) -> Id {
        match self {
            Self { condition, success, failure: None, .. } => {
                state.push_error(
                    condition
                        .location()
                        .into_extended(success.location())
                        .fail("Missing ':' after ternary operator".to_owned()),
                );
                Id::NotFound
            }
            Self { condition, failure: Some((loc, failure)), .. } if failure.is_empty() => {
                state.push_error(
                    condition
                        .location()
                        .into_extended(loc)
                        .fail("Missing node after ':' in ternary operator".to_owned()),
                );
                Id::NotFound
            }
            Self { condition, success, failure: Some((_, failure)) } => {
                let loc_cond = condition.location();
                let loc_succ = success.location();
                let loc_fail = failure.location();
                match (
                    condition.push_in(bbs, state),
                    success.push_in(bbs, state),
                    failure.push_in(bbs, state),
                ) {
                    res @ ((_, _, None) | (_, None, _) | (None, _, _)) => {
                        if res.0.is_none() {
                            state.stat_not_expr(loc_cond);
                        }
                        if res.1.is_none() {
                            state.stat_not_expr(loc_succ);
                        }
                        if res.2.is_none() {
                            state.stat_not_expr(loc_fail);
                        }
                        Id::NotFound
                    }
                    (Some(Id::NotFound), _, _)
                    | (_, Some(Id::NotFound), _)
                    | (_, _, Some(Id::NotFound)) => Id::NotFound,
                    (Some(Id::Found(cond)), Some(Id::Found(succ)), Some(Id::Found(fail))) => state
                        .push_element(Value::Ternary(cond, succ, fail), vec![])
                        .into(),
                }
            }
        }
    }
}
