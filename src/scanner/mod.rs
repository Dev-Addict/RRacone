mod token;

use std::{iter::Peekable, str::Chars};

use token::{Token, TokenType};

use crate::{
    error::{Error, SyntaxError},
    result::Result,
};

#[derive(Debug)]
pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            tokens: vec![],
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Error> {
        let mut errors = Vec::new();

        while self.chars.peek().is_some() {
            if let Err(e) = self.scan_token() {
                errors.push(e);
            }
        }

        self.add_token(TokenType::Eof);

        errors
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    fn scan_token(&mut self) -> Result<()> {
        if let Some(c) = self.chars.next() {
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
                '!' => {
                    let token_type = if self.is_match('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };

                    self.add_token(token_type)
                }
                '=' => {
                    let token_type = if self.is_match('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };

                    self.add_token(token_type);
                }
                '<' => {
                    let token_type = if self.is_match('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };

                    self.add_token(token_type);
                }
                '>' => {
                    let token_type = if self.is_match('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };

                    self.add_token(token_type);
                }
                '/' => {
                    if self.is_match('/') {
                        for c in self.chars.by_ref() {
                            if c == '\n' {
                                break;
                            }
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                '"' => self.string()?,
                c if c.is_ascii_digit() => self.number(c)?,
                c if c.is_alphabetic() || c == '_' => self.identifier(c),
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,
                _ => {
                    return Err(Error::Syntax(SyntaxError::UnexpectedCharacter {
                        line: self.line,
                        character: c,
                    }));
                }
            }
        }

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, self.line))
    }

    fn is_match(&mut self, expected: char) -> bool {
        if let Some(c) = self.chars.peek() {
            if *c == expected {
                self.chars.next();

                return true;
            }
        }

        false
    }

    fn string(&mut self) -> Result<()> {
        let mut buf = String::new();

        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                break;
            }

            if c == '\n' {
                self.line += 1;
            }

            buf.push(c);
            self.chars.next();
        }

        if let Some(c) = self.chars.peek() {
            if *c != '"' {
                return Err(Error::Syntax(SyntaxError::UnterminatedString {
                    line: self.line,
                }));
            }
        }

        self.chars.next();
        self.add_token(TokenType::String(buf));

        Ok(())
    }

    fn number(&mut self, start_char: char) -> Result<()> {
        let mut buf = start_char.to_string();

        while let Some(&c) = self.chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }

            buf.push(c);
            self.chars.next();
        }

        if let Some(c) = self.chars.peek() {
            if *c == '.' {
                self.chars.next();

                if let Some(c) = self.chars.peek() {
                    if c.is_ascii_digit() {
                        buf.push('.');

                        while let Some(&c) = self.chars.peek() {
                            if !c.is_ascii_digit() {
                                break;
                            }

                            buf.push(c);
                            self.chars.next();
                        }

                        self.add_token(TokenType::Number(buf.parse::<f64>().or(Err(
                            Error::Syntax(SyntaxError::InvalidNumber { line: self.line }),
                        ))?));

                        return Ok(());
                    }
                }

                self.add_token(TokenType::Number(buf.parse::<f64>().or(Err(
                    Error::Syntax(SyntaxError::InvalidNumber { line: self.line }),
                ))?));
                self.add_token(TokenType::Dot);

                return Ok(());
            }
        }

        self.add_token(TokenType::Number(buf.parse::<f64>().or(Err(
            Error::Syntax(SyntaxError::InvalidNumber { line: self.line }),
        ))?));

        Ok(())
    }

    fn identifier(&mut self, start_char: char) {
        let mut buf = start_char.to_string();

        while let Some(&c) = self.chars.peek() {
            if !(c.is_alphanumeric() || c == '_') {
                break;
            }

            buf.push(c);
            self.chars.next();
        }

        self.add_token(match buf.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier(buf),
        });
    }
}
