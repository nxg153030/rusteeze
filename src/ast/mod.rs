pub mod expression;
pub mod statement;

pub use expression::Expression;
pub use statement::Statement;

pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub enum AST {
    Number(i32),
    BinaryOp {
        op: Operator, // The operator (e.g., "+", "-", "*", "/")
        left: Box<AST>, // The left operand (another AST node)
        right: Box<AST>, // The right operand (another AST node)
    }
}