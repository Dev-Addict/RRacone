use std::{error, fmt::Display, io};

#[derive(Debug)]
pub enum InterpreterError {
    Io(io::Error),
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Io Error: {e}"),
        }
    }
}

impl error::Error for InterpreterError {}

impl From<io::Error> for InterpreterError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
