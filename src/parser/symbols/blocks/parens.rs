//! Define the [`Ast`] nodes that start with parenthesis
//!
//! This includes casts, compound literals and simple parenthesis blocks.

use core::{fmt, mem};

use crate::parser::display::repr_fullness;
use crate::parser::literal::{Attribute, repr_vec_attr};
use crate::parser::operators::api::OperatorConversions;
use crate::parser::tree::api::Ast;
use crate::parser::variable::api::PureType;
use crate::utils::display;

/// Cast and Compound Literals
///
/// Cast and compound literals do not differ on the syntax (if you ignore that
/// a cast take an expression and a compound literal takes a
/// [`ListInitialiser`](crate::parser::symbols::api::ListInitialiser)), only on
/// the implementation.
#[derive(Debug)]
pub struct Cast {
    /// Type to cast to
    pub dest_type: Vec<Attribute>,
    /// Fullness of the cast expression
    pub full: bool,
    /// Value
    pub value: Box<Ast>,
}

impl Cast {
    /// Convert a [`ParensBlock`] (containing the type) and an [`Ast`]
    /// (containing the expression to be casted) to a valid [`Cast`] if it is
    /// possible.
    pub fn parens_node_into_cast(parens: &mut ParensBlock, new: &mut Ast) -> Option<Ast> {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::custom_print(&format!(
            "Trying to make cast of parens {parens} & ast {new}"
        ));
        if matches!(
            new,
            Ast::Empty
                | Ast::Binary(_)
                | Ast::Ternary(_)
                | Ast::ControlFlow(_)
                | Ast::FunctionCall(_)
                | Ast::FunctionArgsBuild(_)
                | Ast::BracedBlock(_)
        ) {
            None
        } else if matches!(new, Ast::ListInitialiser(_)) {
            parens.take_pure_type().map(|dest_type| {
                Ast::Cast(Self { dest_type, full: false, value: mem::take(new).into_box() })
            })
        } else {
            let full = matches!(new, Ast::Cast(_) | Ast::ListInitialiser(_) | Ast::ParensBlock(_));
            parens.take_pure_type().map(|dest_type| {
                Ast::Cast(Self { dest_type, full, value: mem::take(new).into_box() })
            })
        }
    }

    /// See [`Operator::precedence`](crate::parser::operators::api::Operator::precedence)
    pub const fn precedence() -> u32 {
        2
    }
}

display!(
    Cast,
    self,
    f,
    write!(
        f,
        "(({})\u{b0}{}{})",
        repr_vec_attr(&self.dest_type),
        &self.value,
        repr_fullness(self.full)
    )
);

/// Struct to represent parenthesis
///
/// The [`Ast`] is what is inside of the parenthesis.
///
///
/// If the C source is `(x = 2)`, the node is a [`ParensBlock`] with value the
/// [`Ast`] of `x=2`.
#[derive(Debug, Default)]
pub struct ParensBlock(Box<Ast>);

impl ParensBlock {
    /// Adds parenthesis around an [`Ast`].
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert!(ParensBlock::make_parens_ast(Ast::Empty) == Ast::ParensBlock(Ast::empty_box()));
    /// ```
    pub fn make_parens_ast(node: Ast) -> Ast {
        Ast::ParensBlock(Self(node.into_box()))
    }

    /// Method to push an [`Operator`](crate::parser::operators::api::Operator)
    /// into a [`ParensBlock`]
    ///
    /// This method handles cases of cast and non-cast operators.
    ///
    /// See
    /// [`Push::push_op`](crate::parser::modifiers::push::Push::push_op) for
    /// more information on operator pushing.
    ///
    /// # Note
    ///
    /// In the code of this function, the [`OperatorConversions::try_to_node`]
    /// must be called called before taking the value of parens. Indeed, if it
    /// fails, [`Push::push_op`](crate::parser::modifiers::push::Push::push_op)
    /// is called again on the same [`Ast`] with a unary operator instead of
    /// binary, and we need the [`ParensBlock`] to still contains its value. But
    /// it must also fail only if parens is a *pure type* (see
    /// [`Variable::take_pure_type`](crate::parser::variable::Variable)), for
    /// instance not to miss that (a+b)*c is meant as a
    /// [`BinaryOperator`](crate::parser::operators::api::BinaryOperator)! Hence
    /// the usage of [`ParensBlock::is_pure_type`] before
    /// [`ParensBlock::take_pure_type`].
    pub fn take_ast_with_op<T>(&mut self, op: T) -> Result<Ast, String>
    where
        T: OperatorConversions + Copy + fmt::Display,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "parens");
        if self.is_pure_type() {
            let node_op = op.try_to_node()?;
            Ok(Ast::Cast(Cast {
                dest_type: self.take_pure_type().expect("just checked if possible"),
                full: false,
                value: node_op.into_box(),
            }))
        } else {
            let mut ast = Ast::ParensBlock(mem::take(self));
            op.try_push_op_as_root(&mut ast)?;
            Ok(ast)
        }
    }
}

impl PureType for ParensBlock {
    fn is_pure_type(&self) -> bool {
        if let Ast::Variable(var) = &*self.0
            && var.is_pure_type()
        {
            true
        } else {
            false
        }
    }

    fn take_pure_type(&mut self) -> Option<Vec<Attribute>> {
        if let Ast::Variable(var) = &mut *self.0 {
            var.take_pure_type()
        } else {
            None
        }
    }
}

display!(ParensBlock, self, f, write!(f, "({})", self.0));
