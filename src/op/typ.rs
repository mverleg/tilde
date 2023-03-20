use ::std::any::Any;
use ::std::borrow::Cow;
use ::std::fmt;
use ::std::hash;
use ::std::ops::Deref;

use crate::compile::GolfWord;
use crate::exec::Executor;

pub struct Op {
    // This only allocates for the Ops with data, i.e. literals
    val: Box<dyn OpTyp>,
    //TODO @mark: Rc
}

impl Op {
    pub fn of(op: impl OpTyp + 'static) -> Self {
        Op { val: Box::new(op) }
    }
}

pub trait OpTyp: fmt::Debug + OpClone + OpEq {

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
    fn clone_op(&self) -> Op;
}

impl<T> OpClone for T where T: 'static + OpTyp + Clone {
    fn clone_op(&self) -> Op {
        Op { val: Box::new(self.clone()) }
    }
}

impl Clone for Op {
    fn clone(&self) -> Self {
        self.val.clone_op()
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

impl hash::Hash for Op {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        //TODO @mark: self.hash(state)
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.val)
    }
}
