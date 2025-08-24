mod interpreter;

use std::{error, fmt::Display, io};

pub use interpreter::InterpreterError;

#[derive(Debug)]
pub enum Error {
    Interpreter(InterpreterError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Interpreter(e) => write!(f, "Interpreter Error: {e}"),
        }
    }
}

impl error::Error for Error {}

impl From<InterpreterError> for Error {
    fn from(value: InterpreterError) -> Self {
        Self::Interpreter(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Interpreter(value.into())
    }
}
