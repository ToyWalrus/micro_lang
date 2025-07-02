use micro_lang::{Interpreter, Lexer, Parser, SemanticAnalyzer, VM};

fn main() {
    let input = r#"
    x = (10 + 5 * 2) / 4;
    y = x + 10;
    z = y - y / 5;
    "#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let mut analyzer = SemanticAnalyzer::new();

    let program = &parser.parse_program().ok().unwrap();
    _ = analyzer.analyze(program);

    let mut interpreter = Interpreter::new();
    let instructions = interpreter.generate_instructions(program);

    let mut vm = VM::new(instructions, analyzer.symbol_table);

    println!("{:?}", vm.execute());
}
