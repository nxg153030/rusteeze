use crate::ast::{Operator, AST};

pub struct Codegen;

impl Codegen {
    pub fn new() -> Self {
        Codegen
    }

    pub fn generate_code(&self, ast: &AST) -> String {
        match ast {
            AST::Number(value) => value.to_string(),
            AST::BinaryOp {op, left, right} => {
                let left_code = self.generate_code(left);
                let right_code = self.generate_code(right);
                let operator = match op {
                    Operator::Plus => "+",
                    Operator::Minus => "-",
                    Operator::Multiply => "*",
                    Operator::Divide => "/",
                };
                format!("({} {} {})", left_code, operator, right_code)
            }
        }
     }
}