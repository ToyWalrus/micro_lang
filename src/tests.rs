use crate::ast::*;
use crate::lexer::*;
use crate::parser::Parser;

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
