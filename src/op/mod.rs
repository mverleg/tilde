use ::std::borrow::Cow;
use ::std::fmt::Debug;

use ::tinyvec::ArrayVec;

use crate::compile::Letter;
use crate::op::literal::Number;

mod literal;

#[derive(Debug)]
pub struct Op {
    // This only allocates for the Ops with data, i.e. literals
    val: Box<dyn OpTyp>,
}

pub trait OpTyp: Debug {

    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str;

    fn long_code(&self) -> Option<Cow<str>>;

    fn golf_code(&self) -> Option<ArrayVec<Letter>>;

    //TODO @mark: evaluation methods
}

pub fn all_non_literals() -> [&'static Op; 1] {
    [Number.into()]
}


//TODO @mark: long and gold not both empty
//TODO @mark: id unique and sequential
//TODO @mark: name unique and identifier-safe
