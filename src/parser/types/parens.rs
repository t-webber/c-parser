//! Define the [`Ast`] nodes that start with parenthesis
//!
//! This includes casts, compound literals and simple parenthesis blocks.

use core::{fmt, mem};

use super::Ast;
use super::literal::{Attribute, repr_vec_attr};
use super::operator::{Associativity, Operator};
use crate::parser::modifiers::conversions::OperatorConversions;
use crate::parser::repr_fullness;

/// Cast and Compound Literals
///
/// Cast and compound literals do not differ on the syntax (if you ignore that
/// a cast take an expression and a compound literal takes a
/// [`ListInitialiser`](super::ListInitialiser)), only on the implementation.
#[derive(Debug, PartialEq)]
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
                Ast::Cast(Self {
                    dest_type,
                    full: false,
                    value: mem::take(new).into_box(),
                })
            })
        } else {
            let full = matches!(
                new,
                Ast::Cast(_) | Ast::ListInitialiser(_) | Ast::ParensBlock(_)
            );
            parens.take_pure_type().map(|dest_type| {
                Ast::Cast(Self {
                    dest_type,
                    full,
                    value: mem::take(new).into_box(),
                })
            })
        }
    }
}

impl Operator for Cast {
    fn associativity(&self) -> Associativity {
        Associativity::RightToLeft
    }

    fn precedence(&self) -> u32 {
        2
    }
}

impl OperatorConversions for Cast {
    fn try_to_node(self) -> Result<Ast, String> {
        Ok(Ast::Cast(self))
    }

    fn try_to_node_with_arg(self, arg: Ast) -> Result<Ast, String> {
        if self.value.is_empty() {
            Ok(Ast::Cast(Self {
                value: arg.into_box(),
                ..self
            }))
        } else {
            Err("Tried to add an argument to cast, but it already has one.".to_owned())
        }
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for Cast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})\u{b0}{}{}",
            repr_vec_attr(&self.dest_type),
            &self.value,
            repr_fullness(self.full)
        )
    }
}

/// Struct to represent parenthesis
///
/// The [`Ast`] is what is inside of the parenthesis.
///
///
/// If the C source is `(x = 2)`, the node is a [`ParensBlock`] with value the
/// [`Ast`] of `x=2`.
#[derive(Debug, Default, PartialEq)]
pub struct ParensBlock(Box<Ast>);

impl ParensBlock {
    /// Checks if the parenthesis block can become a cast if followed by a
    /// variable.
    ///
    /// # Returns
    ///
    /// This method returns `true` iff the parenthesised block can become a
    /// cast, i.e., iff it contains a *pure type* variable. See []
    pub fn can_become_cast(&self) -> bool {
        if let Ast::Variable(var) = &*self.0
            && var.is_pure_type()
        {
            true
        } else {
            false
        }
    }

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

    /// Method to push an [`Operator`] into a [`ParensBlock`]
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
    /// [`Variable::take_pure_type`](super::variable::Variable)), for instance
    /// not to miss that (a+b)*c is meant as a
    /// [`BinaryOperator`](super::binary::BinaryOperator)! Thus the usage
    /// of [`ParensBlock::can_become_cast`] before
    /// [`ParensBlock::take_pure_type`].
    pub fn take_ast_with_op<T>(&mut self, op: T) -> Result<Ast, String>
    where
        T: OperatorConversions + Copy + fmt::Display,
    {
        #[cfg(feature = "debug")]
        crate::errors::api::Print::push_op(&op, self, "parens");
        if self.can_become_cast() {
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

    /// Checks if the parenthesis block can become a cast if followed by a
    /// variable.
    ///
    /// # Returns
    ///
    /// This method returns `true` iff the parenthesised block can become a
    /// cast, i.e., iff it contains a *pure type* variable. See
    /// [`Variable::take_pure_type`](super::variable::Variable::take_pure_type).
    pub fn take_pure_type(&mut self) -> Option<Vec<Attribute>> {
        if let Ast::Variable(var) = &mut *self.0 {
            var.take_pure_type()
        } else {
            None
        }
    }
}

#[expect(clippy::min_ident_chars)]
#[coverage(off)]
impl fmt::Display for ParensBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}
