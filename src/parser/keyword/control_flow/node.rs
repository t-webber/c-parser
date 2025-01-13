//! Defines the control flow nodes.

use core::{fmt, mem};

use super::keyword::ControlFlowKeyword;
use super::typedef::Typedef;
use crate::EMPTY;
use crate::parser::modifiers::ast::AstPushContext;
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::modifiers::push::Push;
use crate::parser::types::braced_blocks::BracedBlock;
use crate::parser::types::{Ast, ParensBlock};
use crate::parser::{repr_fullness, repr_option, repr_vec};

/// Node representation of a control flow.
#[expect(clippy::option_option, reason = "//TODO: ctrl flow refactor needed")]
#[derive(Debug, PartialEq)]
pub enum ControlFlowNode {
    /// Keyword expects a node: `return 3+4`
    Ast(ControlFlowKeyword, Box<Ast>, bool),
    /// Keyword expects a colon and a node: `goto: label` or `default`
    AstColonAst(ControlFlowKeyword, Box<Ast>, Option<Box<Ast>>, bool),
    /// Keyword expects a node and then a colon: `case 2:`
    ColonAst(ControlFlowKeyword, Option<Box<Ast>>, bool),
    /// Keywords expected a colon then a identifier: `goto: label`
    ColonIdent(ControlFlowKeyword, bool, Option<String>),
    /// `if` keyword
    Condition(Option<ParensBlock>, Box<Ast>, bool, Option<Box<Ast>>, bool),
    /// `do` keyword
    DoWhile(Box<Ast>, Option<Option<ParensBlock>>),
    /// Keyword expects an identifier and a braced block: `struct Blob {}`
    IdentBlock(ControlFlowKeyword, Option<String>, Option<BracedBlock>),
    /// Keyword expects a parenthesised block and a braced block: `switch (cond)
    /// {};`
    ParensBlock(ControlFlowKeyword, Option<ParensBlock>, Box<Ast>, bool),
    /// Keyword expects a semicolon: `break;`
    SemiColon(ControlFlowKeyword),
    /// Typedef control flow: `typedef struct`
    Typedef(Typedef),
}

impl ControlFlowNode {
    /// Sets the control flow to full
    pub fn fill(&mut self) {
        match self {
            Self::Ast(.., full)
            | Self::ColonAst(.., full)
            | Self::Condition(.., full, _, false)
            | Self::Condition(.., true, _, full)
            | Self::AstColonAst(.., full)
            | Self::ParensBlock(.., full) => {
                *full = true;
            }
            Self::DoWhile(..)
            | Self::SemiColon(..)
            | Self::ColonIdent(..)
            | Self::IdentBlock(..)
            | Self::Typedef(..) => (),
            Self::Condition(_, _, false, _, true) => panic!("unreachable"),
        }
    }

    /// Function to return an [`Ast`], if exists
    pub const fn get_ast(&self) -> Option<&Ast> {
        match self {
            Self::Ast(_, ast, false)
            | Self::DoWhile(ast, None)
            | Self::ParensBlock(.., ast, false)
            | Self::Condition(_, ast, false, ..)
            | Self::ColonAst(_, Some(ast), false)
            | Self::AstColonAst(_, ast, None, false)
            | Self::AstColonAst(.., Some(ast), false)
            | Self::Condition(_, _, true, Some(ast), false) => Some(ast),

            Self::Typedef(..)
            | Self::SemiColon(_)
            | Self::Condition(..)
            | Self::Ast(.., true)
            | Self::ColonIdent(..)
            | Self::IdentBlock(..)
            | Self::ColonAst(.., true)
            | Self::DoWhile(_, Some(_))
            | Self::ColonAst(_, None, _)
            | Self::ParensBlock(.., true)
            | Self::AstColonAst(.., true) => None,
        }
    }
    /// Function to return an [`Ast`], if exists
    pub fn get_ast_mut(&mut self) -> Option<&mut Ast> {
        match self {
            Self::Ast(_, ast, false)
            | Self::DoWhile(ast, None)
            | Self::ParensBlock(.., ast, false)
            | Self::Condition(_, ast, false, ..)
            | Self::ColonAst(_, Some(ast), false)
            | Self::AstColonAst(_, ast, None, false)
            | Self::AstColonAst(.., Some(ast), false)
            | Self::Condition(_, _, true, Some(ast), false) => Some(ast),
            Self::Typedef(..)
            | Self::SemiColon(_)
            | Self::Ast(.., true)
            | Self::Condition(..)
            | Self::IdentBlock(..)
            | Self::ColonIdent(..)
            | Self::ColonAst(.., true)
            | Self::DoWhile(_, Some(_))
            | Self::ColonAst(_, None, _)
            | Self::ParensBlock(.., true)
            | Self::AstColonAst(.., true) => None,
        }
    }

    /// Get keyword from node
    pub const fn get_keyword(&self) -> &ControlFlowKeyword {
        match self {
            Self::Typedef(_) => &ControlFlowKeyword::Typedef,
            Self::DoWhile(..) => &ControlFlowKeyword::Do,
            Self::Condition(..) => &ControlFlowKeyword::If,
            Self::Ast(keyword, ..)
            | Self::SemiColon(keyword)
            | Self::ColonAst(keyword, ..)
            | Self::IdentBlock(keyword, ..)
            | Self::ColonIdent(keyword, ..)
            | Self::AstColonAst(keyword, ..)
            | Self::ParensBlock(keyword, ..) => keyword,
        }
    }

    /// Checks if the control flow is complete
    ///
    /// Complete means that the control flow means something on its own, no
    /// addition data is required. It doesn't mean you can't push data in it
    /// anymore, it just means you don't have to.
    pub const fn is_complete(&self) -> bool {
        if let Self::Condition(.., full_s, failure, full_f) = self {
            *full_f || (*full_s && failure.is_none())
        } else {
            self.is_full()
        }
    }

    /// Checks if the control flow is full
    ///
    /// Full means that nothing can be pushed inside anymore
    pub const fn is_full(&self) -> bool {
        match self {
            Self::SemiColon(_) | Self::DoWhile(_, Some(Some(_))) => true,
            Self::DoWhile(..) => false,
            Self::ColonIdent(_, _, ident) => ident.is_some(),
            Self::Condition(.., full_s, _, full_f) => *full_f && *full_s,
            Self::Typedef(typedef) => typedef.is_full(),
            Self::ParensBlock(_, parens, _, full) => parens.is_some() && *full,
            Self::IdentBlock(_, ident, node) => node.is_some() && ident.is_some(),
            Self::Ast(.., full) | Self::ColonAst(.., full) | Self::AstColonAst(.., full) => *full,
        }
    }

    /// Pushes a block as leaf in a [`ControlFlowNode::IdentBlock`].
    ///
    /// See [`ControlFlowNode::push_block_as_leaf`] for more information.
    fn push_block_as_leaf_in_ident_block(
        values: (
            &mut ControlFlowKeyword,
            &mut Option<String>,
            &mut Option<BracedBlock>,
        ),
        node: Ast,
    ) -> Result<(), String> {
        match (values.1, values.2, node) {
            (_, Some(_), ast) => panic!("Tried to push {ast} on full control flow."),
            (_, block @ None, Ast::BracedBlock(braced)) => *block = Some(braced),
            (ident @ None, None, Ast::Variable(var)) => {
                *ident = Some(var.into_user_defined_name()?);
            }
            (Some(_), None, Ast::Variable(_)) => {
                return Err(
                    "Found 2 successive variable: expected block after variable.".to_owned(),
                );
            }
            (
                _,
                _,
                ast @ (Ast::Empty
                | Ast::Leaf(_)
                | Ast::Unary(_)
                | Ast::Binary(_)
                | Ast::Ternary(_)
                | Ast::ParensBlock(_)
                | Ast::ControlFlow(_)
                | Ast::FunctionCall(_)
                | Ast::ListInitialiser(_)
                | Ast::FunctionArgsBuild(_)),
            ) => {
                return Err(format!(
                    "Tried to push invalid leaf to struct definition. Expected block or name, found {ast}"
                ));
            }
        }
        Ok(())
    }

    /// Tries to push a colon inside the control flow node.
    pub fn push_colon(&mut self) -> Result<(), String> {
        if let Self::AstColonAst(_, _, ast @ None, false) = self {
            *ast = Some(Box::new(Ast::Empty));
            Ok(())
        } else if let Self::ColonAst(_, ast @ None, false) = self {
            *ast = Some(Box::new(Ast::Empty));
            Ok(())
        } else if let Self::ColonIdent(_, found_col @ false, ..) = self {
            *found_col = true;
            Ok(())
        } else if let Some(child) = self.get_ast_mut() {
            Self::push_colon_in_node_for_control_flow(child)
        } else {
            Err(format!(
                "Found extra colon: illegal in control flow keyword context: Tried to push colon in {self}."
            ))
        }
    }

    /// Tries to push a colon inside an [`Ast`], but only to push it inside a
    /// control flow.
    #[expect(clippy::option_if_let_else, reason = "see issue #13964")]
    fn push_colon_in_node_for_control_flow(ast: &mut Ast) -> Result<(), String> {
        match ast {
            Ast::Empty
            | Ast::Leaf(_)
            | Ast::Unary(_)
            | Ast::Binary(_)
            | Ast::Ternary(_)
            | Ast::Variable(_)
            | Ast::ParensBlock(_)
            | Ast::FunctionCall(_)
            | Ast::ListInitialiser(_)
            | Ast::FunctionArgsBuild(_)
            | Ast::BracedBlock(BracedBlock { full: true, .. }) => Err(format!(
                "Found extra colon: illegal in control flow keyword context: Tried to push colon in {ast}."
            )),
            Ast::BracedBlock(BracedBlock { elts, full: false }) => match elts.last_mut() {
                Some(last) => Self::push_colon_in_node_for_control_flow(last),
                None => Err(format!(
                    "Found extra colon: illegal in control flow keyword context: Tried to push colon in {ast}."
                )),
            },
            Ast::ControlFlow(ctrl) => ctrl.push_colon(),
        }
    }
}

#[expect(clippy::too_many_lines, reason = "//TODO: ctrl flow refactor needed")]
impl Push for ControlFlowNode {
    fn push_block_as_leaf(&mut self, ast: Ast) -> Result<(), String> {
        #[cfg(feature = "debug")]
        println!("\tPushing {ast} as leaf in ctrl {self}");
        match self {
            Self::Ast(_, arg, full @ false)
            | Self::ColonAst(_, Some(arg), full @ false)
            | Self::AstColonAst(_, arg, None, full @ false)
            | Self::AstColonAst(.., Some(arg), full @ false)
            | Self::Condition(Some(_), _, true, Some(arg), full @ false)
            | Self::Condition(Some(_), arg, full @ false, None, false)
            | Self::ParensBlock(_, Some(_), arg, full @ false) => {
                if matches!(ast, Ast::BracedBlock(_)) {
                    if **arg == Ast::Empty {
                        *arg = Box::new(ast);
                        *full = true;
                    } else {
                        arg.push_braced_block(ast)?;
                        if !arg.can_push_leaf_with_ctx(AstPushContext::UserVariable) {
                            *full = true;
                        }
                    }
                } else {
                    arg.push_block_as_leaf(ast)?;
                }
            }
            //
            Self::ParensBlock(keyword, old_parens @ None, _, false) => {
                if let Ast::ParensBlock(ast_parens) = ast {
                    *old_parens = Some(ast_parens);
                } else {
                    return Err(format!(
                        "{keyword} expected a parenthesised block but found {ast}",
                    ));
                }
            }
            Self::Typedef(typedef) => typedef.push_block_as_leaf(ast)?,
            Self::IdentBlock(keyword, ident, block) => {
                Self::push_block_as_leaf_in_ident_block((keyword, ident, block), ast)?;
            }
            Self::Condition(cond @ None, ..) => {
                if let Ast::ParensBlock(parens) = ast {
                    *cond = Some(parens);
                } else {
                    return Err("Missing condition after `if` keyword.".to_owned());
                }
            }
            Self::ColonIdent(_, true, ident @ None) => {
                return if let Ast::Variable(var) = ast {
                    if var.has_empty_attrs() {
                        let mut mutable = var;
                        mutable.take_user_defined().map_or_else(
                            || Err(format!("Invalid label {mutable}")),
                            |name| {
                                *ident = Some(name);
                                Ok(())
                            },
                        )
                    } else {
                        var.into_attrs().map_or_else(|_| Err(
                            "Expected label, but found function keyword which is not allowed.".to_owned(
                            )), |attrs| Err(format!(
                            "Expected label, but found variable declaration. No attributes allowed, but found {}",
                            repr_vec(&attrs)
                            )))
                    }
                } else {
                    Err(format!(
                        "Expected label after `goto` colon, but found illegal ast {ast}"
                    ))
                };
            }
            Self::DoWhile(node, while_found @ None) => {
                if let Ast::ControlFlow(ctrl) = &ast
                    && ctrl.get_keyword() == &ControlFlowKeyword::While
                {
                    debug_assert!(
                        ctrl.get_ast().is_none_or(|x| x == &Ast::Empty),
                        "pushing `while` in `do-while` with formed control flow {} [{}:{}:{}]",
                        ctrl.get_ast().expect("its not none"),
                        file!(),
                        line!(),
                        column!()
                    );
                    *while_found = Some(None);
                } else {
                    node.push_block_as_leaf(ast)?;
                }
            }
            Self::DoWhile(_, Some(condition @ None)) => {
                if let Ast::ParensBlock(parens) = ast {
                    *condition = Some(parens);
                } else {
                    return Err(
                        "Missing condition after while keyword. Expected '(' but not found."
                            .to_owned(),
                    );
                }
            }
            Self::SemiColon(_)
            | Self::Ast(.., true)
            | Self::Condition(..)
            | Self::ParensBlock(..)
            | Self::ColonAst(.., true)
            | Self::AstColonAst(.., true)
            | Self::ColonIdent(.., Some(_))
            | Self::ColonAst(_, None, false)
            | Self::DoWhile(_, Some(Some(_)))
            | Self::ColonIdent(_, false, ..) => {
                panic!("Tried to push not on full block, but it is not pushable")
            }
        }
        Ok(())
    }

    fn push_op<T>(&mut self, op: T) -> Result<(), String>
    where
        T: fmt::Display + OperatorConversions + Copy,
    {
        #[cfg(feature = "debug")]
        println!("\tPushing op {op} in ctrl {self}");
        match self {
            Self::Ast(_, ast, false)
            | Self::DoWhile(ast, None)
            | Self::ColonAst(_, Some(ast), false)
            | Self::AstColonAst(_, ast, None, false)
            | Self::AstColonAst(.., Some(ast), false)
            | Self::ParensBlock(_, Some(_), ast, false)
            | Self::Condition(Some(_), ast, false, None, false)
            | Self::Condition(Some(_), _, true, Some(ast), false) => ast.push_op(op),
            Self::IdentBlock(_, Some(_), Some(BracedBlock { elts, full: false })) => {
                if let Some(last) = elts.last_mut() {
                    last.push_op(op)
                } else {
                    elts.push(op.try_to_node()?);
                    Ok(())
                }
            }
            Self::Typedef(typedef) => typedef.push_op(op),
            Self::SemiColon(_)
            | Self::Ast(.., true)
            | Self::IdentBlock(..)
            | Self::ColonAst(.., true)
            | Self::Condition(None, ..)
            | Self::DoWhile(_, Some(_))
            | Self::ColonIdent(..)
            | Self::ColonAst(_, None, _)
            | Self::AstColonAst(.., true)
            | Self::ParensBlock(.., true)
            | Self::ParensBlock(_, None, ..)
            | Self::Condition(.., true, _, true)
            | Self::Condition(.., true, None, false) => Err(format!(
                "Illegal operator {op} in {} control flow.",
                self.get_keyword()
            )),
            Self::Condition(_, _, false, _, true) | Self::Condition(_, _, false, Some(_), _) => {
                panic!("never happens")
            }
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ControlFlowNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ast(keyword, ast, full) => {
                write!(f, "<{keyword} {ast}{}>", repr_fullness(*full))
            }
            Self::ColonIdent(keyword, found_colon, label) => write!(
                f,
                "<{keyword} {}{}>",
                repr_option(label),
                if *found_colon { ":" } else { "\\:" },
            ),
            Self::ColonAst(keyword, ast, full) => {
                write!(
                    f,
                    "<{keyword}: {}{}>",
                    repr_option(ast),
                    repr_fullness(*full)
                )
            }
            Self::AstColonAst(keyword, before, after, full) => {
                write!(
                    f,
                    "<{keyword} {}: {}{}>",
                    before,
                    repr_option(after),
                    repr_fullness(*full)
                )
            }
            Self::Typedef(typedef) => typedef.fmt(f),
            Self::IdentBlock(keyword, ident, block) => write!(
                f,
                "<{keyword} {} {}>",
                repr_option(ident),
                repr_option(block)
            ),
            Self::ParensBlock(keyword, parens_block, ast, full) => {
                write!(
                    f,
                    "<{keyword} {} {ast}{}>",
                    repr_option(parens_block),
                    repr_fullness(*full)
                )
            }
            Self::SemiColon(keyword) => write!(f, "<{keyword}>"),
            Self::Condition(cond, success, full_s, failure, full_f) => write!(
                f,
                "<if {} {success}{}{}{}>",
                repr_option(cond),
                if *full_s { "" } else { ".." },
                failure
                    .as_ref()
                    .map_or_else(String::new, |ast| format!(" else {ast}")),
                if *full_f { "" } else { ".\u{b2}." }
            ),
            Self::DoWhile(block, cond) => {
                write!(
                    f,
                    "<do {block} {} {}>",
                    if cond.is_some() { "while" } else { EMPTY },
                    repr_option(&cond.as_ref().and_then(|x| x.as_ref()))
                )
            }
        }
    }
}

/// Find if the current [`Ast`] corresponds to an unclosed `switch` control
/// flow, waiting for the block.
///
/// This function is called when reading `{` to see whether
pub fn switch_wanting_block(current: &Ast) -> bool {
    match current {
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::Unary(_)
        | Ast::Binary(_)
        | Ast::Ternary(_)
        | Ast::Variable(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(_)
        | Ast::BracedBlock(BracedBlock { full: true, .. }) => false,
        Ast::ControlFlow(ControlFlowNode::ParensBlock(
            ControlFlowKeyword::Switch,
            Some(_),
            _,
            false,
        )) => true,
        Ast::ControlFlow(ctrl) => ctrl.get_ast().is_some_and(switch_wanting_block),
        Ast::BracedBlock(BracedBlock { full: false, elts }) => {
            elts.last().is_some_and(switch_wanting_block)
        }
    }
}

/// Try to push a semicolon into a control flow.
///
/// Adding a semicolon makes the state of a condition move one, by marking the
/// first piece full.
pub fn try_push_semicolon_control(current: &mut Ast) -> bool {
    match current {
        Ast::Empty
        | Ast::Leaf(_)
        | Ast::Unary(_)
        | Ast::Binary(_)
        | Ast::Ternary(_)
        | Ast::Variable(_)
        | Ast::ParensBlock(_)
        | Ast::FunctionCall(_)
        | Ast::ListInitialiser(_)
        | Ast::FunctionArgsBuild(_) => false,
        Ast::ControlFlow(
            ControlFlowNode::Condition(_, ast, full @ false, ..)
            | ControlFlowNode::Condition(_, _, true, Some(ast), full @ false)
            | ControlFlowNode::Ast(_, ast, full @ false),
        ) => {
            if try_push_semicolon_control(ast) {
                if !ast.can_push_leaf() {
                    *full = true;
                }
            } else {
                ast.fill();
                *full = true;
            }
            true
        }
        Ast::ControlFlow(ControlFlowNode::Condition(_, _, true, None, full @ false)) => {
            *full = true;
            true
        }
        Ast::ControlFlow(
            ControlFlowNode::ColonAst(_, Some(ast), false)
            | ControlFlowNode::AstColonAst(_, _, Some(ast), false),
        ) => {
            if let Ast::BracedBlock(BracedBlock { elts, full: false }) = &mut **ast {
                elts.push(Ast::Empty);
            } else {
                **ast = Ast::BracedBlock(BracedBlock {
                    elts: vec![mem::take(ast), Ast::Empty],
                    full: false,
                });
            }
            true
        }
        Ast::ControlFlow(ctrl) => ctrl.get_ast_mut().is_some_and(try_push_semicolon_control),
        Ast::BracedBlock(BracedBlock { elts, full }) => {
            !*full && elts.last_mut().is_some_and(try_push_semicolon_control)
        }
    }
}
