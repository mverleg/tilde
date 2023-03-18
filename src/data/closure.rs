use ::std::hash;
use ::std::hash::Hasher;

use crate::op::Op;

#[derive(Debug, Clone)]
pub struct Func {
    ops: Vec<Op>,
    //TODO @mark: tinyvec?
}

impl Func {
    pub fn new() -> Self {
        Func { ops: Vec::with_capacity(4) }
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
