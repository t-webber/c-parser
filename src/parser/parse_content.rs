//! Module to convert a list of [`Token`] into an [`Ast`].

extern crate alloc;
use alloc::vec::IntoIter;

use super::keyword::handle_keyword;
use super::literal::Literal;
use super::modifiers::push::Push as _;
use super::state::ParsingState;
use super::symbols::api::BracedBlock;
use super::symbols::handle_symbol;
use super::tree::api::Ast;
use super::variable::Variable;
use crate::EMPTY;
use crate::errors::api::{ErrorLocation, Res};
use crate::lexer::api::{Token, TokenValue};
use crate::parser::api::{Binary, FunctionCall, Ternary, Unary};
use crate::parser::keyword::control_flow::traits::ControlFlow as _;
use crate::parser::operators::api::{Associativity, Operator as _};
use crate::parser::symbols::api::{Cast, ListInitialiser};
use crate::utils::{StringResolver, repr_fullness};

/// Indicates whether the current block should continue parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseAction {
    /// Continue parsing the current block.
    Continue,
    /// Stop parsing the current block (a closing delimiter was found).
    Stop,
}

impl StringResolver<BracedBlock> {
    /// Function to display a tree in a user-readable format.
    ///
    /// # Examples
    ///
    /// ```
    /// use c_parser::*;
    ///
    /// let tokens = parse(
    ///     lex("int f(char x) { return a+b+c; }", 0)
    ///         .unwrap_or_display(&[])
    ///         .unwrap(),
    /// )
    /// .unwrap_or_display(&[])
    /// .display();
    /// assert!(&displayed == "int $f(char $x) { return ((a+b)+c) }");
    /// ```
    #[must_use]
    pub fn display(&self) -> String {
        self.display_bb(&self.as_value().elts, self.as_value().full)
    }

    /// Displays a basic block.
    pub(super) fn display_bb(&self, elts: &[Ast], full: bool) -> String {
        format!("[{}{}]", self.display_vec(elts), repr_fullness(full))
    }

    /// Displays one node.
    pub(super) fn display_node(&self, node: &Ast) -> String {
        match node {
            Ast::Binary(Binary { op, arg_l, arg_r }) =>
                format!("({}{op}{})", self.display_node(arg_l), self.display_node(arg_r.as_ref())),
            Ast::BracedBlock(bb) => self.display_bb(&bb.elts, bb.full),
            Ast::Cast(Cast { dest_type, full, value, .. }) => format!(
                "({}\u{2190}{}{})",
                self.display_type(dest_type.as_slice(), |attr| attr.as_value()),
                self.display_node(value),
                repr_fullness(*full)
            ),
            Ast::ControlFlow(ctrl) => format!("<{}>", ctrl.display(self)),
            Ast::Empty => EMPTY.to_owned(),
            Ast::FunctionArgsBuild(elts, ..) => format!("({})", self.display_vec(elts)),
            Ast::FunctionCall(FunctionCall { arguments, function_body, variable, .. }) => format!(
                "({}°({}){})",
                variable.display(self),
                self.display_vec(arguments),
                function_body
                    .as_ref()
                    .map(|body| self.display_bb(&body.elts, body.full))
                    .unwrap_or_default()
            ),
            Ast::Leaf(lit) => self.display_lit(lit.as_value()),
            Ast::ListInitialiser(ListInitialiser { elts, full, .. }) =>
                format!("{{{}{}}}", self.display_vec(elts), repr_fullness(*full)),
            Ast::ParensBlock(parens) => format!("({})", self.display_node(parens.as_value())),
            Ast::Ternary(Ternary { condition, failure, success }) => format!(
                "({}?{}:{})",
                self.display_node(condition),
                self.display_node(success),
                if let Some((_, fail)) = failure {
                    self.display_node(fail)
                } else {
                    EMPTY.to_owned()
                }
            ),
            Ast::Unary(Unary { arg, op }) =>
                if op.associativity() == Associativity::LeftToRight {
                    format!("({}{op})", self.display_node(arg))
                } else {
                    format!("({op}{})", self.display_node(arg))
                },

            Ast::Variable(var) => format!("({})", var.display(self)),
        }
    }

    /// Displays a list of nodes, separated by a comma.
    fn display_vec(&self, vec: &[Ast]) -> String {
        vec.iter()
            .map(|node| self.display_node(node))
            .collect::<Vec<_>>()
            .join(",")
    }
}

/// Pushes a [`Literal`] into the [`Ast`]
fn handle_literal(current: &mut Ast, lit: Ast, location: ErrorLocation) -> Res<ParseAction> {
    current
        .push_block_as_leaf(lit)
        .map_err(|err| location.crash(err))?;
    Res::ok(ParseAction::Continue)
}

/// Function to parse one node, and by recursivity, one block. At the end of the
/// block, this function stops and is recalled from [`parse`].
pub fn parse_block(
    tokens: &mut IntoIter<Token>,
    p_state: &mut ParsingState,
    current: &mut Ast,
) -> Res<()> {
    let mut errors = vec![];
    while let Some(token) = tokens.next() {
        #[cfg(feature = "debug")]
        println!("\x1b[36m{:20} on {current}\x1b[0m", format!("{token}"),);
        let (value, location) = token.into_value_location();
        let res = match value {
            TokenValue::Char(ch) =>
                handle_literal(current, Ast::Leaf(location.wrap(Literal::Char(ch))), location),
            TokenValue::Ident(val) =>
                handle_literal(current, Ast::Variable(Variable::from(location.wrap(val))), location),
            TokenValue::Number(nb) =>
                handle_literal(current, Ast::Leaf(location.wrap(Literal::Number(nb))), location),
            TokenValue::Str(val) =>
                handle_literal(current, Ast::Leaf(location.wrap(Literal::Str(val))), location),
            TokenValue::Symbol(symbol) => handle_symbol(symbol, current, p_state, tokens, location),
            TokenValue::Keyword(keyword) => handle_keyword(keyword, current, p_state, location),
        };
        let has_failures = res.has_failures();
        let action = res.store_errors(&mut |err| errors.push(err));
        if has_failures || action != Some(ParseAction::Continue) {
            break;
        }
    }
    Res::from(((), errors))
}

/// Parses a list of tokens into an Abstract Syntax Tree.
///
/// This function manages the blocks with successive calls and checks.
#[must_use]
pub fn parse(lex_result: StringResolver<Vec<Token>>) -> Res<StringResolver<BracedBlock>> {
    lex_result.map_res(|tokens| {
        let mut tokens_iter = tokens.into_iter();
        let mut ast = Ast::BracedBlock(BracedBlock::default());
        let mut p_state = ParsingState::default();
        let res = parse_block(&mut tokens_iter, &mut p_state, &mut ast);
        let Ast::BracedBlock(braced_block) = ast else {
            unreachable!("Braced block can't become another node.")
        };
        if !res.has_failures() && p_state.has_opening_blocks() {
            res.extend_errs(p_state.mismatched_error())
                .map(|()| braced_block)
        } else {
            res.map(|()| braced_block)
        }
    })
}
