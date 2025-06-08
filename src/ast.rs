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

// Helper function to convert Token to BinaryOperator
pub fn token_to_binary_op(token: crate::lexer::Token) -> Result<BinaryOperator, String> {
    use crate::lexer::Token;
    match token.clone() {
        Token::Plus => Ok(BinaryOperator::Add),
        Token::Minus => Ok(BinaryOperator::Subtract),
        Token::Multiply => Ok(BinaryOperator::Multiply),
        Token::Divide => Ok(BinaryOperator::Divide),
        other => Err(format!("Token {:?} was not a binary operator", other)),
    }
}
