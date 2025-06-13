use std::collections::HashMap;

use crate::interpreter::Instruction;
use crate::semantic_analyzer::SymbolTable;

pub struct VM {
    instructions: Vec<Instruction>,
    symbol_table: SymbolTable,
    storage: HashMap<String, f64>,
    program_counter: usize,
    stack: Vec<f64>,
}

impl VM {
    pub fn new(instructions: Vec<Instruction>, symbol_table: SymbolTable) -> VM {
        let storage = HashMap::new();
        VM {
            instructions,
            symbol_table,
            storage,
            program_counter: 0,
            stack: vec![],
        }
    }

    pub fn execute(&mut self) -> HashMap<String, f64> {
        self.storage.clear();
        self.program_counter = 0;
        self.stack = vec![];

        while self.program_counter < self.instructions.len() {
            self.evaluate_next_instruction();
            self.program_counter += 1;
        }

        self.storage.clone()
    }

    fn evaluate_next_instruction(&mut self) {
        if let Some(instruction) = self.instructions.get(self.program_counter) {
            match instruction {
                Instruction::LoadConstant(x) => {
                    self.stack.push(x.clone());
                }
                Instruction::LoadVariable(x) => {
                    if let None = self.symbol_table.lookup_variable(x) {
                        panic!("Variable not in scope! ({})", x)
                    }

                    if let Some(val) = self.storage.get(x) {
                        self.stack.push(val.clone());
                    } else {
                        panic!("Variable was in scope, but somehow not in storage! ({})", x)
                    }
                }
                Instruction::StoreVariable(x) => {
                    if let Some(val) = self.stack.pop() {
                        self.storage.insert(x.clone(), val);
                    } else {
                        panic!("Stack is empty, cannot store variable \"{}\" !", x)
                    }
                }
                Instruction::Add => {
                    let (n1, n2) = self.pop_two();
                    self.stack.push(n2 + n1);
                }
                Instruction::Subtract => {
                    let (n1, n2) = self.pop_two();
                    self.stack.push(n2 - n1);
                }
                Instruction::Multiply => {
                    let (n1, n2) = self.pop_two();
                    self.stack.push(n2 * n1);
                }
                Instruction::Divide => {
                    let (n1, n2) = self.pop_two();
                    if n2 == 0.0 {
                        panic!("Cannot divide by zero!")
                    }
                    self.stack.push(n2 / n1);
                }
                Instruction::Stop => {
                    // println!("Program done, current stack is {:?}", self.stack)
                }
            }
        }
    }

    fn pop_two(&mut self) -> (f64, f64) {
        let n1 = self.stack.pop();
        let n2 = self.stack.pop();

        if n1.is_some() && n2.is_some() {
            return (n1.unwrap(), n2.unwrap());
        }

        panic!("Unable to pop two from stack!")
    }
}
