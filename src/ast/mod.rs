pub mod expression;
pub mod statement;

pub use expression::Expression;
pub use statement::Statement;

pub enum AST {
    Number(i32),
    BinaryOp {
        op: String,
        left: Box<AST>,
        right: Box<AST>,
    }
}