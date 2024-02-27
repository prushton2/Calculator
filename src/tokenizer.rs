enum TokenType {
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Times,
    Divide,
    Number,
    Exponent,
}

struct Token  {
    pub token: TokenType,
    pub lexeme: String
}

impl Token {
    pub fn tokenize(string: String) -> Result<Vec<Token>, String> {
        
        let mut i = 0;

        


        return Err(String::from("nope"));
    }
}
