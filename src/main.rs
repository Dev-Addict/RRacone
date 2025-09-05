mod ast;
mod error;
mod parser;
mod result;
mod scanner;

use std::{
    env::args,
    fs,
    io::{self, Write, stdin},
};

use parser::Parser;
use result::Result;
use scanner::Scanner;

fn run(source: String) -> Result<()> {
    let mut scanner = Scanner::new(source.as_str());

    scanner.scan_tokens();

    let mut parser = Parser::new(scanner.tokens());

    match parser.parse() {
        Ok(expression) => {
            dbg!(expression);
        }
        Err(e) => {
            dbg!(e);
        }
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
