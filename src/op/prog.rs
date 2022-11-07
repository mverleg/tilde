pub use ::std::slice;
use ::std::vec;

use crate::op::mathbool::Bool2Op;
use crate::op::mathbool::CompareOp;
use crate::op::mathbool::Math1Op;
use crate::op::mathbool::Math2Op;
use crate::op::typ::Typ;
use crate::op::value::ValueOp;
use crate::op::Op;

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
