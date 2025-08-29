use std::fmt::Debug;

use crate::scanner::token::Token;

#[derive(Clone)]
pub enum Expr {
    Literal(Literal),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal:?}"),
            Self::Unary { operator, right } => write!(f, "(\\{} {right:?})", operator.token_type()),
            Self::Binary {
                left,
                operator,
                right,
            } => write!(f, "(\\{} {left:?} {right:?})", operator.token_type()),
            Self::Grouping(expr) => write!(f, "(group {expr:?})"),
        }
    }
}

#[derive(Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::String(s) => write!(f, "\"{s}\""),
            Self::Bool(val) => write!(f, "{}", if *val { "true" } else { "false" }),
            Self::Nil => write!(f, "nil"),
        }
    }
}
