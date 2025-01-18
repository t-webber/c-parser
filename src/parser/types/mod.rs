//! Module that defines the main node types of the [`Ast`]

pub mod binary;
pub mod braced_blocks;
pub mod literal;
pub mod operator;
pub mod ternary;
pub mod unary;
pub mod variable;

use core::{fmt, mem};

use binary::Binary;
use braced_blocks::BracedBlock;
use literal::{Attribute, Literal, repr_vec_attr};
use operator::{Associativity, Operator};
use ternary::Ternary;
use unary::Unary;
use variable::Variable;

use super::keyword::control_flow::node::ControlFlowNode;
use super::modifiers::conversions::OperatorConversions;
use crate::parser::{repr_fullness, repr_vec};

/// Struct to represent the Abstract Syntax Tree of the whole C source file.
///
/// # Note
///
/// Can't derive [`Eq`] because it is not implemented for [`f32`].
#[derive(Debug, Default, PartialEq)]
pub enum Ast {
    /// Binary operator
    Binary(Binary),
    /// Braced-block, in `{...}`.
    ///
    /// A whole file is considered to be a block.
    BracedBlock(BracedBlock),
    /// Cast
    Cast(Cast),
    /// Control Flow blocks
    ControlFlow(ControlFlowNode),
    /// Empty AST
    #[default]
    Empty,
    /// Function arguments: `(x+y, !g(z), (a, !b)++, )`
    FunctionArgsBuild(Vec<Ast>),
    /// Function call
    FunctionCall(FunctionCall),
    /// Literal (constants, variables, etc.)
    Leaf(Literal),
    /// List initialiser: `{1, 2, 3, [6]=7}`
    ListInitialiser(ListInitialiser),
    /// Ast surrounded by parenthesis: `(x=2)`
    ParensBlock(ParensBlock),
    /// Ternary operator
    Ternary(Ternary),
    /// Unary operator
    Unary(Unary),
    /// Variables
    Variable(Variable),
}

/// Cast and Compound Literals
///
/// Cast and compound literals do not differ on the syntax (if you ignore that
/// a cast take an expression and a compound literal takes a
/// [`ListInitialiser`]), only on the implementation.
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
                    full: true,
                    value: Box::new(mem::take(new)),
                })
            })
        } else {
            let full = !matches!(new, Ast::Unary(_));
            parens.take_pure_type().map(|dest_type| {
                Ast::Cast(Self {
                    dest_type,
                    full,
                    value: Box::new(mem::take(new)),
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
        if *self.value == Ast::Empty {
            Ok(Ast::Cast(Self {
                value: Box::new(arg),
                ..self
            }))
        } else {
            Err("Tried to add an argument to cast, but it already has one.".to_owned())
        }
    }
}

#[expect(clippy::min_ident_chars)]
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

/// Function call
///
/// This node represents functions declaration, functions
#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    /// arguments of the function
    pub args: Vec<Ast>,
    /// Function operator
    ///
    /// This is a constant type, but is used to access the methods of the
    /// [`Operator`] trait.
    pub op: FunctionOperator,
    /// name of the function, and all its attributes (return type)
    pub variable: Variable,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}\u{b0}({}))", self.variable, repr_vec(&self.args),)
    }
}

/// Function operator
///
/// This is a constant type, but is used to access the methods of the
/// [`Operator`] trait.
#[derive(Debug, PartialEq, Eq)]
pub struct FunctionOperator;

impl Operator for FunctionOperator {
    fn associativity(&self) -> Associativity {
        Associativity::LeftToRight
    }

    fn precedence(&self) -> u32 {
        1
    }
}

/// List initialiser
///
/// Node to represent list initialisers, such as `{1, 2, 3, [6]=12}`.
#[derive(Debug, PartialEq, Default)]
pub struct ListInitialiser {
    /// elements of the list
    pub elts: Vec<Ast>,
    /// indicates whether the closing `}` was found yet.
    ///
    /// If full is false, we can still push elements inside.
    pub full: bool,
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ListInitialiser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.elts
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// Struct to represent parenthesis
///
/// The [`Ast`] is what is inside of the parenthesis.
///
/// # Examples
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
        Ast::ParensBlock(Self(Box::new(node)))
    }

    /// Method to push an [`Operator`] into a [`ParensBlock`]
    ///
    /// This method handles cases of cast and non-cast operators.
    ///
    /// See
    /// [`Push::push_op`](super::modifiers::push::Push::push_op) for more
    /// information on operator pushing.
    ///
    /// # Note
    ///
    /// In the code of this function, the [`OperatorConversions::try_to_node`]
    /// must be called called before taking the value of parens. Indeed, if it
    /// fails, [`Push::push_op`](super::modifiers::push::Push::push_op) is
    /// called again on the same [`Ast`] with a unary operator instead of
    /// binary, and we need the [`ParensBlock`] to still contains its value. But
    /// it must also fail only if parens is a *pure type* (see
    /// [`Variable::take_pure_type`]), for instance not to miss that (a+b)*c
    /// is meant as a [`BinaryOperator`](binary::BinaryOperator)! Thus the usage
    /// of [`ParensBlock::can_become_cast`] before
    /// [`ParensBlock::take_pure_type`].
    pub fn take_ast_with_op<T>(&mut self, op: T) -> Result<Ast, String>
    where
        T: OperatorConversions + Copy,
    {
        if self.can_become_cast() {
            let node_op = op.try_to_node()?;
            if op.is_valid_lhs()
                && let Some(dest_type) = self.take_pure_type()
            {
                Ok(Ast::Cast(Cast {
                    dest_type,
                    full: false,
                    value: Box::new(node_op),
                }))
            } else {
                panic!("just checked if possible")
            }
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
    /// [`Variable::take_pure_type`].
    pub fn take_pure_type(&mut self) -> Option<Vec<Attribute>> {
        if let Ast::Variable(var) = &mut *self.0 {
            var.take_pure_type()
        } else {
            None
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for ParensBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}
