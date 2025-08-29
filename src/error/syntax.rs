use std::{error, fmt::Display};

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedCharacter { line: usize, character: char },
    UnterminatedString { line: usize },
    InvalidNumber { line: usize },
    UnterminatedMultilineComment { line: usize },
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedCharacter { line, character } => {
                write!(f, "Unexpected Character at Line {line}: {character}")
            }
            Self::UnterminatedString { line } => write!(f, "Unterminated String at Line: {line}"),
            Self::UnterminatedMultilineComment { line } => {
                write!(f, "Unterminated Multiline Comment at Line: {line}")
            }
            Self::InvalidNumber { line } => write!(f, "Invalid Number at Line: {line}"),
        }
    }
}

impl error::Error for SyntaxError {}
