// Assignment 1: Define the basic token structure for MicroLang

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Divide,
    Multiply,
    Assign,
    LParen,
    RParen,
    Semi,
    EoF,
}

fn main() {
    let ident_token = Token::Identifier("HI".to_string());
    println!("Token: {:?}", ident_token);
    
    println!("MiniLang tokenizer initialized!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        assert_eq!(Token::Assign, Token::Assign);
        assert_eq!(Token::Number(4.), Token::Number(4.0));
        assert_eq!(Token::Identifier("val".to_string()), Token::Identifier("val".to_string()))
    }
}