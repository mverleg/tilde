use ::std::fmt::Write;
use ::std::ops::Index;
pub use ::std::slice;
use ::std::vec;

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
        let mut code = String::with_capacity(self.ops.len() * 16);
        for op in &self.ops {
            write!(code, "{} ", op.long_code().as_ref()).unwrap()  //TODO @mverleg:
        }
        code.pop();
        code
    }

    pub fn golf_code(&self) -> String {
        let mut code = String::with_capacity(self.ops.len() * 4);
        for op in &self.ops {
            write!(code, "{}", op.golf_code().as_ref()).unwrap()  //TODO @mverleg:
        }
        code
    }
}
