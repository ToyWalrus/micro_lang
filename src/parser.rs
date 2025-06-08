use crate::ast::{boxed_node, token_to_binary_op, ASTNode};
use crate::lexer::{Lexer, Token};

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
        self.parse_term()
    }

    fn parse_assignment(&mut self) -> Result<ASTNode, String> {
        let ident_token = self.expect_identifier_token()?;
        let ident = match ident_token {
            Token::Identifier(name) => name,
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
        self.parse_assignment()
    }

    pub fn parse_program(&mut self) -> Result<ASTNode, String> {
        let mut program_vec: Vec<ASTNode> = vec![];
        while self.current_token != Token::EoF {
            program_vec.push(self.parse_statement()?);
        }

        Ok(ASTNode::Program(program_vec))
    }
}
