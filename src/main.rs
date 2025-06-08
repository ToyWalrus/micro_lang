use crate::lexer::Lexer;
use crate::parser::Parser;

mod ast;
mod lexer;
mod parser;

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
