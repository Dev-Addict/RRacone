mod ast;
mod error;
mod result;
mod scanner;

use std::{
    env::args,
    fs,
    io::{self, Write, stdin},
};

use result::Result;
use scanner::Scanner;

use crate::{
    ast::{Expr, Literal},
    scanner::token::{Token, TokenType},
};

fn run(source: String) -> Result<()> {
    let mut scanner = Scanner::new(source.as_str());

    scanner.scan_tokens();

    for token in scanner.tokens() {
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
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, 1),
            right: Box::new(Expr::Literal(Literal::Number(123.0))),
        }),
        operator: Token::new(TokenType::Star, 1),
        right: Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
            45.67,
        ))))),
    };

    dbg!(expr);

    if args().len() > 2 {
        println!("Usage: rracone [script]");
    } else if args().len() == 2 {
        return run_file(args().nth(1).unwrap());
    } else {
        return run_prompt();
    }

    Ok(())
}
