use std::{error, fmt::Display};

use crate::scanner::token::{Token, TokenType};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEnd {
        token: Token,
    },
    ExpectedAnother {
        expected_token_type: TokenType,
        token: Token,
    },
    ExpectedOther {
        token: Token,
    },
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEnd { token } => {
                write!(f, "Expected expression: {}", token.line())
            }
            Self::ExpectedAnother {
                expected_token_type,
                token,
            } => write!(
                f,
                "Expected {} instead got {} at line: {}",
                expected_token_type,
                token.token_type(),
                token.line()
            ),
            Self::ExpectedOther { token } => write!(
                f,
                "Unexpected token {} at line: {}",
                token.token_type(),
                token.line()
            ),
        }
    }
}

impl error::Error for ParserError {}
