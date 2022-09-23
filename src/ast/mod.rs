pub use ::std::slice;

pub use crate::ast::mathbool::{Bool2Op, CompareOp, Math1Op, Math2Op};
pub use crate::ast::value::ValueOp;

mod mathbool;
mod optype;
mod value;

#[derive(Debug)]
pub struct Prog {
    ops: Vec<Op>,
}

impl Prog {
    pub fn of(ops: Vec<Op>) -> Self {
        Prog { ops }
    }

    pub fn iter(&self) -> slice::Iter<Op> {
        self.ops.iter()
    }
}

#[derive(Debug)]
pub enum Op {
    Math1(Math1Op),
    Math2(Math2Op),
    Compare(CompareOp),
    Bool2(Bool2Op),
    Value(ValueOp),
}

impl Op {
    pub fn name(&self) -> &str {
        match self {
            Op::Math1(op) => op.name(),
            Op::Math2(op) => op.name(),
            Op::Compare(op) => op.name(),
            Op::Bool2(op) => op.name(),
            Op::Value(op) => op.name(),
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Op::Math1(op) => op.description(),
            Op::Math2(op) => op.description(),
            Op::Compare(op) => op.description(),
            Op::Bool2(op) => op.description(),
            Op::Value(op) => op.description(),
        }
    }
}