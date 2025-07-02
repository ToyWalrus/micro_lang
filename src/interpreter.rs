use crate::ast::{ASTNode, BinaryOperator};

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadConstant(f64),
    LoadVariable(String),
    StoreVariable(String),
    Add,
    Subtract,
    Divide,
    Multiply,

    // End program
    Stop,
}

pub struct Interpreter {
    instructions: Vec<Instruction>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let operations: Vec<Instruction> = vec![];
        Interpreter {
            instructions: operations,
        }
    }

    pub fn generate_instructions(&mut self, program: &ASTNode) -> Vec<Instruction> {
        self.instructions.clear();
        self.visit_node(program);
        self.instructions.push(Instruction::Stop);
        self.instructions.clone()
    }

    fn visit_node(&mut self, node: &ASTNode) {
        let instruction = match node {
            ASTNode::Number(x) => Some(Instruction::LoadConstant(x.clone())),
            ASTNode::Identifier(x) => Some(Instruction::LoadVariable(x.clone())),
            ASTNode::BinaryOp { left, op, right } => {
                self.visit_node(left);
                self.visit_node(right);
                Some(self.binary_op_to_instruction(op))
            }
            ASTNode::Assignment { variable, value } => {
                self.visit_node(value);
                Some(Instruction::StoreVariable(variable.to_string()))
            }
            ASTNode::Program(program) => {
                for node in program {
                    self.visit_node(node);
                }
                None
            }
        };

        if let Some(x) = instruction {
            self.instructions.push(x);
        }
    }

    fn binary_op_to_instruction(&self, op: &BinaryOperator) -> Instruction {
        match op {
            BinaryOperator::Add => Instruction::Add,
            BinaryOperator::Subtract => Instruction::Subtract,
            BinaryOperator::Multiply => Instruction::Multiply,
            BinaryOperator::Divide => Instruction::Divide,
        }
    }
}
