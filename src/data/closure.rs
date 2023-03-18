use ::std::hash;
use ::std::hash::Hasher;

use crate::op::Op;
use crate::Value;

#[derive(Debug, Clone)]
pub enum FuncItem {
    Operation(Op),
    Capture(Value),
}

#[derive(Debug, Clone)]
pub struct Func {
    ops: Vec<FuncItem>,
    //TODO @mark: tinyvec?
}

impl Func {
    pub fn new() -> Self {
        Func { ops: Vec::with_capacity(4) }
    }

    pub fn with_op(mut self, op: Op) -> Self {
        self.ops.push(FuncItem::Operation(op));
        self
    }

    pub fn with_val(mut self, val: Value) -> Self {
        self.ops.push(FuncItem::Capture(val));
        self
    }
}

impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for Func {}

impl hash::Hash for Func {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}
