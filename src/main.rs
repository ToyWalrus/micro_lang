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

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();

        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) -> bool {
        // Increment position and update current_char
        // If we reach the end, set current_char to None
        if self.position >= self.input.len() {
            self.current_char = None;
            return false;
        }

        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
        true
    }

    fn skip_whitespace(&mut self) {
        // Use a while loop and the advance() method
        // Check if current_char is whitespace using char methods
        while !self.current_char.is_none() && char::is_whitespace(self.current_char.unwrap()) {
            self.advance();
        }
    }

    fn read_number(&mut self) -> f64 {
        // Build a string of digits and decimal points
        // Convert the final string to f64 using parse()
        // Handle the case where parsing might fail
        let mut num_string = String::new();
        let mut encountered_decimal = false;

        let result = loop {
            if char::is_numeric(self.current_char.unwrap_or(' ')) {
                num_string.push(self.current_char.unwrap());
            } else if self.current_char.unwrap_or_default() == '.' {
                if encountered_decimal {
                    break num_string;
                }

                encountered_decimal = true;
                num_string.push('.')
            } else {
                break num_string;
            }

            self.advance();
        };

        if result.len() == 0 {
            panic!(
                "Malformed NUMBER, cannot read at index {index} for string \"{string}\"! ({chr})",
                index = self.position,
                string = String::from_iter(&self.input),
                chr = self.current_char.unwrap_or('?')
            );
        }

        result.parse::<f64>().unwrap_or(0.0)
    }

    fn read_identifier(&mut self) -> String {
        // TODO: Read letters and underscores to form an identifier
        // Continue until you hit a non-alphanumeric character
        // Return the collected string
        let mut ident_string = String::new();
        let mut first_iter = true;

        let result = loop {
            let valid = match self.current_char {
                Some(x) if char::is_alphabetic(x) => true,
                Some(x) if char::is_numeric(x) && !first_iter => true,
                Some('_') => true,
                _ => false,
            };

            if valid {
                ident_string.push(self.current_char.unwrap());
            } else {
                break ident_string;
            }

            first_iter = false;
            self.advance();
        };

        if result.len() == 0 {
            panic!(
                "Malformed IDENTIFIER, cannot read at index {index} for string \"{string}\"! ({chr})",
                index = self.position,
                string = String::from_iter(&self.input),
                chr = self.current_char.unwrap_or('?')
            );
        }

        result
    }

    fn match_plain_token(&self, token_char: char) -> Result<Token, &'static str> {
        match token_char {
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Multiply),
            '/' => Ok(Token::Divide),
            '=' => Ok(Token::Assign),
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            ';' => Ok(Token::Semi),
            _ => Err("unknown token type"),
        }
    }

    pub fn next_token(&mut self) -> Token {
        // 1. Skip whitespace
        // 2. Check current_char and return appropriate token
        // 3. Handle numbers, identifiers, and single-character operators
        // 4. Return EoF when current_char is None

        self.skip_whitespace();
        match self.current_char {
            Some(x) if self.match_plain_token(x).is_ok() => {
                let token = self.match_plain_token(x).unwrap();
                self.advance();
                token
            }
            Some(x) if char::is_numeric(x) || x == '.' => Token::Number(self.read_number()),
            Some(x) if char::is_alphanumeric(x) || x == '_' => {
                Token::Identifier(self.read_identifier())
            }
            None | _ => Token::EoF,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Number(f64),
    Identifier(String),
    BinaryOp {
        left: Box<ASTNode>,
        op: BinaryOperator,
        right: Box<ASTNode>,
    },
    Assignment {
        variable: String,
        value: Box<ASTNode>,
    },
    Program(Vec<ASTNode>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Helper function to create boxed AST nodes
pub fn boxed_node(node: ASTNode) -> Box<ASTNode> {
    Box::new(node)
}

// Test function for AST creation
fn test_ast_creation() {
    // TODO: Create a sample AST representing: x = 10 + 5 * 2
    let ast = ASTNode::Program(vec![ASTNode::Assignment {
        variable: "x".to_string(),
        value: boxed_node(ASTNode::BinaryOp {
            left: boxed_node(ASTNode::Number(10.)),
            op: BinaryOperator::Add,
            right: boxed_node(ASTNode::BinaryOp {
                left: boxed_node(ASTNode::Number(5.)),
                op: BinaryOperator::Multiply,
                right: boxed_node(ASTNode::Number(2.)),
            }),
        }),
    }]);

    // Print the AST structure to verify correct nesting
    println!("AST creation test completed: {:?}", ast);
}

// Update main function to include AST testing
fn main() {
    let input = "x = 42 + 3.14; (_4f = .4)";
    let mut lexer = Lexer::new(input);

    let mut token: Token = Token::Semi;
    while token != Token::EoF {
        token = lexer.next_token();
        println!("{:?}", token);
    }

    println!("MiniLang lexer test complete!");

    // AST test
    test_ast_creation();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        assert_eq!(Token::Assign, Token::Assign);
        assert_eq!(Token::Number(4.), Token::Number(4.0));
        assert_eq!(
            Token::Identifier("val".to_string()),
            Token::Identifier("val".to_string())
        )
    }

    #[test]
    fn test_lexer_tokenization() {
        let input = "x = 42 + 3.14 * (_4f - .4) / g;";
        let mut lexer = Lexer::new(input);
        let mut tokens: Vec<Token> = vec![];

        tokens.push(lexer.next_token());
        while tokens.last() != Some(&Token::EoF) {
            tokens.push(lexer.next_token());
        }

        assert_eq!(
            tokens,
            vec![
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(42.0),
                Token::Plus,
                Token::Number(3.14),
                Token::Multiply,
                Token::LParen,
                Token::Identifier("_4f".to_string()),
                Token::Minus,
                Token::Number(0.4),
                Token::RParen,
                Token::Divide,
                Token::Identifier("g".to_string()),
                Token::Semi,
                Token::EoF
            ]
        )
    }
}
