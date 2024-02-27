use crate::tokenizer::{TokenType, Token};

pub fn parse(tokens_imm: &Vec<Token>) -> Result<Token, String> {
    let mut tokens = tokens_imm.clone();
    
    let mut prev_len = tokens.len();

    while tokens.len() > 1 {
        insert_mult(&mut tokens);
        parse_parens(&mut tokens);
        parse_exponents(&mut tokens);
        parse_mult(&mut tokens);
        parse_plus(&mut tokens);
        
        if prev_len == tokens.len() {  //if the length of the previous and current vector are the
                                       //same, nothing has happened. This means nothing will ever
                                       //happen and therefore an infinite loop
            return Err("Loop Detected".to_string());
        }   
        prev_len = tokens.len();
    }


    return Ok(tokens[0].clone());
}


fn parse_parens(tokens: &mut Vec<Token>) {
    for i in 0..tokens.len()-1 {
        if  tokens[i].token == TokenType::OpenParen &&
            tokens[i+1].token == TokenType::Number && 
            tokens[i+2].token == TokenType::CloseParen {
             
            tokens.remove(i+2);
            tokens.remove(i);
            return;
        }
    } 
}

fn insert_mult(tokens: &mut Vec<Token>) {
    for i in 0..tokens.len()-1 {
        if tokens[i].token == TokenType::Number &&
           tokens[i+1].token == TokenType::Number {
            
            tokens.insert(i+1, Token{token: TokenType::Multiply, lexeme: String::from("*")});
            return;
        }
    }

}

fn parse_exponents(tokens: &mut Vec<Token>) {
    for i in 0..tokens.len()-1 {
        if  tokens[i].token == TokenType::Number &&
            tokens[i+1].token == TokenType::Exponent && 
            tokens[i+2].token == TokenType::Number {
            

            let base = tokens[i].lexeme.parse::<f32>().unwrap();
            let exponent = tokens[i+2].lexeme.parse::<f32>().unwrap();

            let power = base.powf(exponent);

            tokens.remove(i+2);
            tokens.remove(i+1);
            tokens[i].lexeme = power.to_string();
            return;
        }
    } 
}

fn parse_mult(tokens: &mut Vec<Token>) {
    for i in 0..tokens.len()-1 {
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
            return;
        }
    } 
}

fn parse_plus(tokens: &mut Vec<Token>) {
    for i in 0..tokens.len()-1 {
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
            return;
        }
    } 
}
