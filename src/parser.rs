use crate::tokenizer::{TokenType, Token};

pub fn parse(tokens_imm: &Vec<Token>) -> Result<Token, String> {
    let mut tokens: Vec<Token> = tokens_imm.clone();
    let mut prev_len: usize = tokens.len();
    
    //This is a war crime, however it is the easiest way to prevent oob indexes
    tokens.push( Token {token: TokenType::Error, lexeme: String::from("")});
    tokens.push( Token {token: TokenType::Error, lexeme: String::from("")});


    while tokens.len() > 3 {
        for i in 0..tokens.len() {
            
            println!("---i:{} | {} -2= {}\n---Tokens:",i, tokens.len(), tokens.len()-2);
            
            for j in &tokens {
                print!("{}", j.lexeme);
            }
            println!("---Functions:");
            
            //println!("insert_mult");
            insert_mult(&mut tokens, i);
            if i > tokens.len()-2 {break; }
           // println!("parse_parens {}", i);
            parse_parens(&mut tokens, i);
            if i > tokens.len()-2 {break; }
          //  println!("parse_exponents");
            parse_exponents(&mut tokens, i);
            if i > tokens.len()-2 {break; }
           // println!("parse_mult");
            parse_mult(&mut tokens, i);
            if i > tokens.len()-2 {break; }
           // println!("parse_add");
            parse_plus(&mut tokens, i);
            if i > tokens.len()-2 {break; }
        }
        
    }


    return Ok(tokens[0].clone());
}


fn parse_parens(tokens: &mut Vec<Token>, i: usize) {
    if  tokens[i].token == TokenType::OpenParen &&
        tokens[i+1].token == TokenType::Number && 
        tokens[i+2].token == TokenType::CloseParen {
         

        tokens.remove(i+2);
        tokens.remove(i);
    }
}

fn insert_mult(tokens: &mut Vec<Token>, i: usize) {
    if tokens[i].token == TokenType::Number &&
       tokens[i+1].token == TokenType::OpenParen &&
       tokens[i+2].token == TokenType::Number {
        
        tokens.insert(i+1, Token{token: TokenType::Multiply, lexeme: String::from("*")});
    }

}

fn parse_exponents(tokens: &mut Vec<Token>, i: usize) {
    if  tokens[i].token == TokenType::Number &&
        tokens[i+1].token == TokenType::Exponent && 
        tokens[i+2].token == TokenType::Number {
        
        let base = tokens[i].lexeme.parse::<f32>().unwrap();
        let exponent = tokens[i+2].lexeme.parse::<f32>().unwrap();

        let power = base.powf(exponent);

        tokens.remove(i+2);
        tokens.remove(i+1);
        tokens[i].lexeme = power.to_string();
    }
}

fn parse_mult(tokens: &mut Vec<Token>, i: usize) {
    if  tokens[i].token == TokenType::Number &&
        (tokens[i+1].token == TokenType::Multiply || tokens[i+1].token == TokenType::Divide) && 
        tokens[i+2].token == TokenType::Number {
        

        let operand1 = tokens[i].lexeme.parse::<f32>().unwrap();
        let operand2 = tokens[i+2].lexeme.parse::<f32>().unwrap();

        let product = match tokens[i+1].token {
            TokenType::Multiply => operand1 * operand2,
            TokenType::Divide => operand1 / operand2,
            _ => 0.0,
        };

        tokens.remove(i+2);
        tokens.remove(i+1);
        tokens[i].lexeme = product.to_string();
    }
}

fn parse_plus(tokens: &mut Vec<Token>, i: usize) {
    if  tokens[i].token == TokenType::Number &&
       (tokens[i+1].token == TokenType::Plus || tokens[i+1].token == TokenType::Minus) && 
        tokens[i+2].token == TokenType::Number {
        

        let operand1 = tokens[i].lexeme.parse::<f32>().unwrap();
        let operand2 = tokens[i+2].lexeme.parse::<f32>().unwrap();

        let sum = match tokens[i+1].token {
            TokenType::Plus => operand1 + operand2,
            TokenType::Minus => operand1 - operand2,
            _ => 0.0,
        };

        tokens.remove(i+2);
        tokens.remove(i+1);
        tokens[i].lexeme = sum.to_string();
    }
}
