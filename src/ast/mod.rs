pub use crate::ast::mathbool::{Bool2Op, CompareOp, Math1Op, Math2Op};
pub use crate::ast::value::{ValueOp};

mod mathbool;
mod value;

#[derive(Debug)]
pub struct Prog {
    ops: Vec<Op>
}

#[derive(Debug)]
pub enum Op {
    Math1(Math1Op),
    Math2(Math2Op),
    Compare(CompareOp),
    Bool2(Bool2Op),
    Value(ValueOp),
}
