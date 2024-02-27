mod tokenizer;
mod parser;

use std::process::exit;
use crate::parser::{parse};
use crate::tokenizer::{Token};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let string: String = args[1..].join(" ");

    let tokens: Vec<Token> = match Token::tokenize(&string) {
        Ok(c) => c,
        Err(_) => vec![],
    };

    let parsed: Token = match parse(&tokens) {
        Some(x) => x,
        None => {println!("Error"); exit(1)}
    };

    println!("{}", parsed.lexeme);
}
