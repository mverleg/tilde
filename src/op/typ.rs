use ::std::any::Any;
use ::std::borrow::Cow;
use ::std::fmt::Debug;
use ::std::ops::Deref;

use crate::compile::GolfWord;
use crate::exec::Executor;

#[derive(Debug)]
pub struct Op {
    // This only allocates for the Ops with data, i.e. literals
    val: Box<dyn OpTyp>,
}

impl Op {
    pub fn of(op: impl OpTyp + 'static) -> Self {
        Op { val: Box::new(op) }
    }
}

pub trait OpTyp: Debug + OpClone + OpEq {

    fn description(&self) -> &'static str;

    fn long_code(&self) -> Cow<'static, str>;

    fn golf_code(&self) -> Option<GolfWord>;

    fn as_any(&self) -> &dyn Any;

    //TODO @mark: can we do without this somehow? Maybe Op should already contain executor info?
    fn as_executor(&self) -> Executor;
}

impl Deref for Op {
    type Target = dyn OpTyp;

    fn deref(&self) -> &Self::Target {
        &*self.val
    }
}

// implemented automatically if Clone is derived/impl for an operation
pub trait OpClone {
    fn clone_box(&self) -> Box<dyn OpTyp>;
}

impl<T> OpClone for T where T: 'static + OpTyp + Clone {
    fn clone_box(&self) -> Box<dyn OpTyp> {
        Box::new(self.clone())
    }
}

impl Clone for Op {
    fn clone(&self) -> Self {
        Op { val: self.val.clone_box() }
    }
}

// implemented automatically if PartialEq + Eq are derived/impl for an operation
pub trait OpEq {
    fn is_equal(&self, other: &dyn OpTyp) -> bool;
}

impl<T> OpEq for T where T: 'static + OpTyp + PartialEq + Eq {
    fn is_equal(&self, other: &dyn OpTyp) -> bool {
        other.as_any()
            .downcast_ref::<Self>()
            .map_or(false, |other_cast| self == other_cast)
    }
}

impl PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        self.val.is_equal(&*other.val)
    }
}

impl Eq for Op {}

