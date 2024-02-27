mod tokenizer;
mod parser;

use std::process::exit;
use crate::parser::{parse};
use crate::tokenizer::{Token};

fn main() {
    let string: String = String::from("(12.32 + 32.333^2 * (1+3))");

    let tokens: Vec<Token> = match Token::tokenize(&string) {
        Ok(c) => c,
        Err(_) => vec![],
    };

    let parsed: Token = match parser::parse(&tokens) {
        Some(x) => x,
        None => {println!("Error"); exit(1)}
    };

    println!("Answer: {}", parsed.lexeme);
}
