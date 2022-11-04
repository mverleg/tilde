pub use ::std::slice;
use ::std::vec;

pub use crate::op::mathbool::Bool2Op;
pub use crate::op::mathbool::CompareOp;
pub use crate::op::mathbool::Math1Op;
pub use crate::op::mathbool::Math2Op;
use crate::op::typ::Typ;
pub use crate::op::value::ValueOp;

mod mathbool;
mod typ;
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

    pub fn into_iter(self) -> vec::IntoIter<Op> {
        self.ops.into_iter()
    }
}

#[derive(Debug)]
pub enum Op {
    Value(ValueOp),
    Math1(Math1Op),
    Math2(Math2Op),
    Compare(CompareOp),
    Bool2(Bool2Op),
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

    pub fn description(
        &self,
        typ: Typ,
    ) -> &str {
        match self {
            Op::Math1(op) => op.description(typ),
            Op::Math2(op) => op.description(typ),
            Op::Compare(op) => op.description(typ),
            Op::Bool2(op) => op.description(typ),
            Op::Value(op) => op.description(typ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_str_repr_bijective {
        ($enm:ident) => {
            todo!()
        };
    }

    #[test]
    fn repr_is_bijective() {
        test_str_repr_bijective!(Op);
    }
}
