use ::std::borrow::Cow;
use ::std::fmt::Debug;
use ::std::ops::Deref;
use ::std::any::Any;

use crate::compile::GolfWord;

pub use self::literal::NumberOp;
pub use self::literal::TextOp;
pub use self::arithmetic::Div;
pub use self::arithmetic::IntDiv;

mod literal;
mod arithmetic;

#[derive(Debug, Clone)]
pub struct Op {
    // This only allocates for the Ops with data, i.e. literals
    val: Box<dyn OpTyp>,
}

impl Op {
    pub fn of(op: impl OpTyp + 'static) -> Self {
        Op { val: Box::new(op) }
    }
}

pub trait OpTyp: Debug + Clone {

    fn description(&self) -> &'static str;

    fn long_code(&self) -> Cow<str>;

    fn golf_code(&self) -> Option<GolfWord>;

    fn as_any(&self) -> &dyn Any;

    fn is_equal(&self, other: &dyn OpTyp) -> bool;

    //TODO @mark: evaluation methods
}

impl PartialEq for Op {
    fn eq(&self, other: &Op) -> bool {
        self.val.is_equal(&*other.val)
    }
}

impl Deref for Op {
    type Target = dyn OpTyp;

    fn deref(&self) -> &Self::Target {
        &*self.val
    }
}

pub fn all_non_literals() -> [Op; 2] {
    //TODO @mark:
    [
        Op::of(Div),
        Op::of(IntDiv),
    ]
}


//TODO @mark: long and gold not both empty
//TODO @mark: id unique and sequential
//TODO @mark: name unique and identifier-safe
