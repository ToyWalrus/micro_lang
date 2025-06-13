use crate::ast::ASTNode;
use std::{collections::HashMap, mem};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Function,
    // Add more types as needed
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: Type,
    pub scope_level: usize,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    current_scope: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let scopes: Vec<HashMap<String, Symbol>> = vec![HashMap::new()];
        let current_scope: usize = 0;
        SymbolTable {
            scopes,
            current_scope,
        }
    }

    pub fn enter_scope(&mut self) {
        self.current_scope += 1;
        while self.scopes.len() <= self.current_scope {
            self.scopes.push(HashMap::new());
        }
    }

    pub fn exit_scope(&mut self) {
        if self.current_scope == 0 {
            panic!("Attempting to pop the final scope!")
        }

        while self.scopes.len() > self.current_scope {
            let _ = self.scopes.pop();
        }
        self.current_scope -= 1;
    }

    pub fn declare_variable(&mut self, name: &String, var_type: Type) -> Result<(), String> {
        let scope = &mut self.scopes[self.current_scope];
        if scope.contains_key(name) {
            return Err(format!(
                "Trying to declare a duplicate variable \"{}\"",
                name
            ));
        }

        scope.insert(
            name.to_string(),
            Symbol {
                name: name.to_string(),
                symbol_type: var_type,
                scope_level: self.current_scope,
            },
        );

        Ok(())
    }

    pub fn lookup_variable(&self, name: &str) -> Option<&Symbol> {
        let mut current_level = self.current_scope;
        while !self.scopes[current_level].contains_key(name) && current_level > 0 {
            current_level -= 1;
        }

        self.scopes[current_level].get(name)
    }
}

#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
    pub error_type: SemanticErrorType,
}

#[derive(Debug, PartialEq)]
pub enum SemanticErrorType {
    UndefinedVariable,
    DuplicateDeclaration,
    TypeMismatch,
}

pub struct SemanticAnalyzer {
    pub symbol_table: SymbolTable,
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let symbol_table = SymbolTable::new();
        let errors: Vec<SemanticError> = vec![];
        SemanticAnalyzer {
            symbol_table,
            errors,
        }
    }

    pub fn analyze(&mut self, ast: &ASTNode) -> Result<(), Vec<SemanticError>> {
        self.visit_node(ast);
        if self.errors.is_empty() {
            Ok(())
        } else {
            // Empty the self.errors vector since we won't need it after returning them
            Err(mem::take(&mut self.errors))
        }
    }

    fn visit_node(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Program(statements) => {
                for node in statements {
                    self.visit_node(node);
                }
            }
            ASTNode::Assignment { variable, value } => {
                let var_type = match self.get_expression_type(value) {
                    Some(x) => x,
                    None => {
                        panic!(
                            "Unable to get expression type of {} ({:?})",
                            variable, value
                        )
                    }
                };

                match self.symbol_table.declare_variable(variable, var_type) {
                    Err(msg) => self.add_error(msg, SemanticErrorType::DuplicateDeclaration),
                    _ => (),
                }
            }
            ASTNode::BinaryOp { left, op: _, right } => {
                self.visit_node(left);
                self.visit_node(right);

                if self.get_expression_type(left) != self.get_expression_type(right) {
                    self.add_error(
                        format!("Type mismatch between operands:\n{:?} | {:?}", left, right),
                        SemanticErrorType::TypeMismatch,
                    );
                }
            }
            ASTNode::Identifier(name) => match self.symbol_table.lookup_variable(name) {
                None => {
                    self.add_error(
                        format!("Variable not in scope: {}", name),
                        SemanticErrorType::UndefinedVariable,
                    );
                }
                _ => (),
            },
            ASTNode::Number(_) => {}
        }
    }

    fn add_error(&mut self, message: String, error_type: SemanticErrorType) {
        self.errors.push(SemanticError {
            message,
            error_type,
        });
    }

    fn get_expression_type(&mut self, node: &ASTNode) -> Option<Type> {
        match node {
            ASTNode::Number(_) => Some(Type::Integer),
            ASTNode::Identifier(name) => match self.symbol_table.lookup_variable(name) {
                Some(x) => Some(x.symbol_type.clone()),
                _ => None,
            },
            ASTNode::BinaryOp {
                left: _,
                op: _,
                right: _,
            } => Some(Type::Integer),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table_basic_operations() {
        let mut table = SymbolTable::new();

        // Initial state
        assert_eq!(table.scopes.len(), 1);
        assert_eq!(table.current_scope, 0);

        // Up scope state
        table.enter_scope();
        assert_eq!(table.scopes.len(), 2);
        assert_eq!(table.current_scope, 1);

        // Down scope state
        table.exit_scope();
        assert_eq!(table.scopes.len(), 1);
        assert_eq!(table.current_scope, 0);

        // Variable declaration
        let var_name = "var1".to_string();
        _ = table.declare_variable(&var_name, Type::Integer);
        assert_eq!(
            table.lookup_variable(&var_name).unwrap().symbol_type,
            Type::Integer
        );

        // Looks up the scopes
        table.enter_scope();
        assert_eq!(
            table.lookup_variable(&var_name).unwrap().symbol_type,
            Type::Integer
        );

        // Variable shadowing
        _ = table.declare_variable(&var_name, Type::Function);
        assert_eq!(
            table.lookup_variable(&var_name).unwrap().symbol_type,
            Type::Function
        );

        // Duplicate declaration
        let result = table.declare_variable(&var_name, Type::Integer);
        assert!(result.is_err());

        // Out-of-scope variables are lost
        _ = table.declare_variable(&"var2".to_string(), Type::Function);
        table.exit_scope();
        table.enter_scope();
        assert_eq!(table.lookup_variable(&"var2".to_string()), None);
    }

    #[test]
    fn test_semantic_analyzer_undefined_variable() {
        let mut analyzer = SemanticAnalyzer::new();
        let result = analyzer.analyze(&ASTNode::Program(vec![ASTNode::Identifier(
            "some_var".to_string(),
        )]));

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap()[0].error_type,
            SemanticErrorType::UndefinedVariable
        )
    }

    #[test]
    fn test_semantic_analyzer_duplicate_declaration() {
        let mut analyzer = SemanticAnalyzer::new();
        let ast = ASTNode::Program(vec![
            ASTNode::Assignment {
                variable: "x".to_string(),
                value: Box::new(ASTNode::Number(1.)),
            },
            ASTNode::Assignment {
                variable: "x".to_string(),
                value: Box::new(ASTNode::Number(2.)),
            },
        ]);

        let result = analyzer.analyze(&ast);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap()[0].error_type,
            SemanticErrorType::DuplicateDeclaration
        );
    }

    #[test]
    fn test_semantic_analyzer_type_mismatch() {
        let mut analyzer = SemanticAnalyzer::new();
        _ = analyzer
            .symbol_table
            .declare_variable(&"x".to_string(), Type::Integer);
        _ = analyzer
            .symbol_table
            .declare_variable(&"y".to_string(), Type::Function);

        let node = ASTNode::BinaryOp {
            left: Box::new(ASTNode::Identifier("x".to_string())),
            op: crate::BinaryOperator::Add,
            right: Box::new(ASTNode::Identifier("y".to_string())),
        };

        let result = analyzer.analyze(&node);

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap()[0].error_type,
            SemanticErrorType::TypeMismatch
        );
    }
}
