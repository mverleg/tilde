pub use ::std::slice;
use ::std::vec;
use std::ops::Index;

use crate::op::Op;
use crate::op::typ::Typ;

#[derive(Debug, PartialEq)]
pub struct Prog {
    ops: Vec<Op>,
}

impl Prog {
    pub fn of(ops: Vec<Op>) -> Self {
        Prog { ops }
    }

    pub fn get(&self, index: usize) -> Option<&Op> {
        self.ops.get(index)
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    // pub fn iter(&self) -> slice::Iter<Op> {
    //     self.ops.iter()
    // }
    //
    // pub fn into_iter(self) -> vec::IntoIter<Op> {
    //     self.ops.into_iter()
    // }
    //TODO @mverleg:
}

impl Index<usize> for Prog {
    type Output = Op;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ops[index]
    }
}

impl Prog {
    pub fn long_code(&self) -> String {
        todo!()
    }

    pub fn golf_code(&self) -> String {
        todo!()
    }
}
