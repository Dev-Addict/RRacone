use crate::{
    Result,
    ast::{Expr, Literal},
    error::{Error, ParserError},
    scanner::token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>> {
        self.expression().inspect_err(|_| {
            self.synchronize();
        })
    }

    fn expression(&mut self) -> Result<Box<Expr>> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.comparison()?;

        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.term()?;

        while self.match_token_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.factor()?;

        while self.match_token_type(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.unary()?;

        while self.match_token_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>> {
        if self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary { operator, right }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>> {
        if self.match_token_type(&[TokenType::False]) {
            return Ok(Box::new(Expr::Literal(Literal::Bool(false))));
        }
        if self.match_token_type(&[TokenType::True]) {
            return Ok(Box::new(Expr::Literal(Literal::Bool(true))));
        }
        if self.match_token_type(&[TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal(Literal::Nil)));
        }

        if self.is_at_end() {
            return Err(Error::Parser(ParserError::UnexpectedEnd {
                token: self.peek().clone(),
            }));
        }

        if let TokenType::Number(num) = self.peek().token_type() {
            let num = *num;
            self.advance();
            return Ok(Box::new(Expr::Literal(Literal::Number(num))));
        }
        if let TokenType::String(s) = self.peek().token_type() {
            let s: String = s.into();
            self.advance();
            return Ok(Box::new(Expr::Literal(Literal::String(s))));
        }

        if self.match_token_type(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(TokenType::RightParen)?;

            return Ok(Box::new(Expr::Grouping(expr)));
        }

        Err(Error::Parser(ParserError::ExpectedOther {
            token: self.peek().clone(),
        }))
    }

    fn consume(&mut self, token_type: TokenType) -> Result<&Token> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(Error::Parser(ParserError::ExpectedAnother {
            expected_token_type: token_type,
            token: self.peek().clone(),
        }))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if *self.previous().token_type() == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type() {
                TokenType::Class
                | TokenType::For
                | TokenType::Fun
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => {
                    return;
                }
                _ => {}
            }
        }

        self.advance();
    }

    fn match_token_type(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type() == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        *self.peek().token_type() == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
