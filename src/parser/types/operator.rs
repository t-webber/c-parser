use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Associativity {
    /// a+b+c is (a+b)+c
    ///
    /// a++-- is (a++)--
    LeftToRight,
    /// a=b=c is a=(b=c)
    ///
    /// !!a is !(!a)
    RightToLeft,
}

#[cfg_attr(doc, doc = include_str!("../../../docs/operators.md"))]
pub trait Operator: fmt::Debug {
    fn associativity(&self) -> Associativity;
    fn precedence(&self) -> u32;
}
