use std::{
    env::args,
    error,
    fmt::Display,
    fs,
    io::{self, Write, stdin},
};

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

#[derive(Debug)]
enum Error {
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

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier(String),
    String(String),
    Number(f64),
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParen => write!(f, "LeftParen"),
            Self::RightParen => write!(f, "RightParen"),
            Self::LeftBrace => write!(f, "LeftBrace"),
            Self::RightBrace => write!(f, "RightBrace"),
            Self::Comma => write!(f, "Comma"),
            Self::Dot => write!(f, "Dot"),
            Self::Minus => write!(f, "Minus"),
            Self::Plus => write!(f, "Plus"),
            Self::Semicolon => write!(f, "Semicolon"),
            Self::Slash => write!(f, "Slash"),
            Self::Star => write!(f, "Star"),
            Self::Bang => write!(f, "Bang"),
            Self::BangEqual => write!(f, "BangEqual"),
            Self::Greater => write!(f, "Greater"),
            Self::GreaterEqual => write!(f, "GreaterEqual"),
            Self::Less => write!(f, "Less"),
            Self::LessEqual => write!(f, "LessEqual"),
            Self::Identifier(identifier) => write!(f, "Identifier({identifier})"),
            Self::String(s) => write!(f, "String(\"s\")"),
            Self::Number(num) => write!(f, "Number({num})"),
            Self::And => write!(f, "And"),
            Self::Class => write!(f, "Class"),
            Self::Else => write!(f, "Else"),
            Self::False => write!(f, "False"),
            Self::Fun => write!(f, "Function"),
            Self::For => write!(f, "For"),
            Self::If => write!(f, "If"),
            Self::Nil => write!(f, "Nil"),
            Self::Or => write!(f, "Or"),
            Self::Print => write!(f, "Print"),
            Self::Return => write!(f, "Return"),
            Self::Super => write!(f, "Super"),
            Self::This => write!(f, "This"),
            Self::True => write!(f, "True"),
            Self::Var => write!(f, "Var"),
            Self::While => write!(f, "While"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}

#[derive(Debug)]
struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    line: usize,
}

impl<'a> Token<'a> {
    fn new(token_type: TokenType, lexeme: &'a str, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}

#[derive(Debug)]
struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn isAtEnd(&self) -> bool {
        self.current > self.source.len()
    }

    fn scanToken(&mut self) {
        if let Some(c) = self.source.chars().peekable().nth(self.current) {
            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),
                _ => {}
            }
        }

        self.current += 1;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            &self.source[self.start..self.current],
            self.line,
        ))
    }
}

fn run(source: String) -> Result<()> {
    for token in source.split_whitespace() {
        println!("{token}")
    }

    Ok(())
}

fn run_file(path: String) -> Result<()> {
    run(fs::read_to_string(path)?)
}

fn run_prompt() -> Result<()> {
    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush()?;

        input.clear();

        let bytes = stdin().read_line(&mut input)?;

        if bytes == 0 {
            break;
        }

        if let Err(e) = run(input) {
            println!("{e}")
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    if args().len() > 2 {
        println!("Usage: rracone [script]");
    } else if args().len() == 2 {
        return run_file(args().nth(1).unwrap());
    } else {
        return run_prompt();
    }

    Ok(())
}
