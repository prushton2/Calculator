use regex::{Regex, Match};
use std::process::exit;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Multiply,
    Divide,
    Number,
    Exponent,
    Semicolon,
    Equals
}

#[derive(Clone)]
pub struct Token  {
    pub token: TokenType,
    pub lexeme: String
}

impl Token {
    pub fn tokenize(string: &String) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![];
        let mut i = 0;
    
        while i < string.len() {
            let slice = &string[i..];

            let is_token = Self::get_token(&slice);
            
            if is_token.is_some() {
                let token = is_token.unwrap();
                i += token.1.clone();
                tokens.push(token.0.clone());
            } else {
                i += 1;
            }
        }
               


        return Ok(tokens);
    }

    fn get_token(string: &str) -> Option<(Token, usize)> {
        
        if &string[0..1] == " " {
            return None;
        }

        let single_char = Regex::new(r"^(\(|\)|\+|-|\*|\/|;|=|\^)").unwrap().find(string);

        if single_char.is_some() {
            let token = match &string[0..1] {
                "(" => TokenType::OpenParen,
                ")" => TokenType::CloseParen,
                "+" => TokenType::Plus,
                "-" => TokenType::Minus,
                "*" => TokenType::Multiply,
                "/" => TokenType::Divide,
                "^" => TokenType::Exponent,
                "=" => TokenType::Equals,
                ";" => TokenType::Semicolon,
                _ => {println!("Error parsing unknown token {}", &string[0..1]); exit(1)}
            };
            return Some((
                    Token {
                        token: token,
                        lexeme: (&string[0..1]).to_string()
                    }, 1));
        }

        let number_regex: Option<Match<'_>> = Regex::new(r"^[0-9][0-9]*(\.|)[0-9]*").unwrap().find(string);
        
        if number_regex.is_some() {
            let number = number_regex.unwrap();
            return Some(( Token{
                token: TokenType::Number,
                lexeme: number.as_str().to_string(),
            }, number.as_str().len() ));
        }

        return None;
    }
}
