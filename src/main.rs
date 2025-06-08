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

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect_number_token(&mut self) -> Result<Token, String> {
        match self.current_token.clone() {
            Token::Number(x) => {
                self.advance();
                Ok(Token::Number(x))
            }
            other => Err(format!(
                "Token ({:?}) was expected to be a Number! (was {:?})",
                self.current_token, other
            )),
        }
    }

    fn expect_identifier_token(&mut self) -> Result<Token, String> {
        match self.current_token.clone() {
            Token::Identifier(x) => {
                self.advance();
                Ok(Token::Identifier(x))
            }
            other => Err(format!(
                "Token ({:?}) was expected to be an Identifier! (was {:?})",
                self.current_token, other
            )),
        }
    }

    fn expect_identifier_or_number_token(&mut self) -> Result<Token, String> {
        match self.current_token.clone() {
            Token::Identifier(_) => self.expect_identifier_token(),
            Token::Number(_) => self.expect_number_token(),
            other => Err(format!(
                "Token ({:?}) was neither Number nor Identifier! (was {:?})",
                self.current_token, other
            )),
        }
    }

    fn expect_operator(&mut self) -> Result<Token, String> {
        if matches!(
            self.current_token,
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide
        ) {
            let token = self.current_token.clone();
            self.advance();
            Ok(token)
        } else {
            Err(format!("Expected operator, got {:?}", self.current_token))
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<Token, String> {
        // If it matches, advance to next token and return Ok(())
        // If not, return an error message
        if self.current_token.clone() == expected {
            self.advance();
            Ok(expected)
        } else {
            Err(format!(
                "Token ({:?}) did not match expected ({:?})",
                self.current_token, expected
            ))
        }
    }

    fn parse_primary(&mut self) -> Result<Box<ASTNode>, String> {
        // Handle Token::Number -> create Number AST node
        // Handle Token::Identifier -> create Identifier AST node
        // Handle Token::LParen -> recursively parse expression, expect RParen
        // Return error for unexpected tokens
        if self.current_token == Token::LParen {
            self.advance();
            let term = self.parse_expression()?;
            self.expect_token(Token::RParen)?;
            return Ok(term);
        }

        match self.expect_identifier_or_number_token()? {
            Token::Identifier(name) => Ok(boxed_node(ASTNode::Identifier(name))),
            Token::Number(val) => Ok(boxed_node(ASTNode::Number(val))),
            other => panic!(
                "parse_primary(): the token {:?} was not LParen, Identifier, or Number! (was {:?})",
                self.current_token, other
            ),
        }
    }

    fn parse_factor(&mut self) -> Result<Box<ASTNode>, String> {
        // Start with parse_primary()
        // While current token is Multiply or Divide:
        //   - Save the operator
        //   - Advance past operator
        //   - Parse right operand with parse_primary()
        //   - Create BinaryOp node
        // This handles operator precedence correctly

        let mut left = self.parse_primary()?;
        while matches!(self.current_token, Token::Multiply | Token::Divide) {
            let op_token = self.expect_operator()?;
            match self.parse_primary() {
                Ok(right) => {
                    left = boxed_node(ASTNode::BinaryOp {
                        left: left.clone(),
                        op: token_to_binary_op(op_token)?,
                        right,
                    });
                }
                Err(err) => {
                    println!("parse_factor(): {}", err);
                    return Ok(left);
                }
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<ASTNode>, String> {
        // Start with parse_factor()
        // While current token is Plus or Minus:
        //   - Save the operator
        //   - Advance past operator
        //   - Parse right operand with parse_factor()
        //   - Create BinaryOp node

        let mut left = self.parse_factor()?;
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op_token = self.expect_operator()?;
            match self.parse_factor() {
                Ok(right) => {
                    left = boxed_node(ASTNode::BinaryOp {
                        left: left.clone(),
                        op: token_to_binary_op(op_token)?,
                        right,
                    });
                }
                Err(err) => {
                    println!("parse_term(): {}", err);
                    return Ok(left);
                }
            };
        }

        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<Box<ASTNode>, String> {
        // For now, this just calls parse_term()
        // Later assignments may add more expression types
        self.parse_term()
    }

    fn parse_assignment(&mut self) -> Result<ASTNode, String> {
        // Expect an identifier for the variable name
        // Expect an assignment token (=)
        // Parse the expression being assigned
        // Handle the semicolon terminator
        // Return Assignment AST node

        let ident_token = self.expect_identifier_token()?;
        let ident = match ident_token {
            Token::Identifier(name) => name,
            // Not throwing another Err since expect_identifier_token() should have already done that
            _ => panic!("Expected identifier, got {:?}", ident_token),
        };

        self.expect_token(Token::Assign)?;

        let expr = self.parse_expression()?;

        let _ = self.expect_token(Token::Semi)?;

        Ok(ASTNode::Assignment {
            variable: ident,
            value: expr,
        })
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        // For now, only handle assignments
        // Check if current token is an identifier (assignment)
        // Return appropriate error for unexpected tokens
        self.parse_assignment()
    }

    pub fn parse_program(&mut self) -> Result<ASTNode, String> {
        // Create a vector to hold statements
        // Loop while not at end of file:
        //   - Parse each statement
        //   - Add to vector
        // Return Program AST node containing all statements
        let mut program_vec: Vec<ASTNode> = vec![];
        while self.current_token != Token::EoF {
            program_vec.push(self.parse_statement()?);
        }

        Ok(ASTNode::Program(program_vec))
    }
}

// Helper function to convert Token to BinaryOperator
fn token_to_binary_op(token: Token) -> Result<BinaryOperator, String> {
    match token.clone() {
        Token::Plus => Ok(BinaryOperator::Add),
        Token::Minus => Ok(BinaryOperator::Subtract),
        Token::Multiply => Ok(BinaryOperator::Multiply),
        Token::Divide => Ok(BinaryOperator::Divide),
        other => Err(format!("Token {:?} was not a binary operator", other)),
    }
}

fn main() {
    let input = "x = 10 + 5 * 2;";
    println!("\nTesting parser with input \"{}\"", input);

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    match parser.parse_program() {
        Ok(ast) => println!("PARSED AST:\n{:?}", ast),
        Err(error) => println!("PARSE ERROR:\n{}", error),
    }
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

    #[test]
    fn test_ast_creation() {
        let input = "x = 10 + 5 * 2;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().ok().unwrap();
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

        assert_eq!(program, ast)
    }
}
