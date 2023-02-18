use ::std::borrow::Cow;
use ::std::fmt::Debug;
use ::std::ops::Deref;
use crate::compile::GolfWord;


use crate::Nr;

pub use self::literal::NumberOp;
pub use self::literal::TextOp;

mod literal;

#[derive(Debug)]
pub struct Op {
    // This only allocates for the Ops with data, i.e. literals
    val: Box<dyn OpTyp>,
}

pub trait OpTyp: Debug {

    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str;

    fn long_code(&self) -> Cow<str>;

    fn golf_code(&self) -> Option<GolfWord>;

    //TODO @mark: evaluation methods
}

impl Deref for Op {
    type Target = dyn OpTyp;

    fn deref(&self) -> &Self::Target {
        &*self.val
    }
}

pub fn all_non_literals() -> [&'static Op; 1] {
    [
        NumberOp::new(Nr::zero()).into(),  //TODO @mark: remove (special only)
    ]
}


//TODO @mark: long and gold not both empty
//TODO @mark: id unique and sequential
//TODO @mark: name unique and identifier-safe
