use ::std::any::Any;
use ::std::borrow::Cow;

use crate::{Array, Nr, Text, Values, values};
use crate::compile::GolfWord;
use crate::exec::{Executor, NullaryExecutor, UnaryExecutor};
use crate::op::{Op, OpTyp};
use crate::op::collection::Split;

#[derive(Debug, Clone, PartialEq)]
pub struct Arg;

impl Arg {
    pub fn new() -> Op {
        Op::of(Arg)
    }
}

impl OpTyp for Arg {

    fn description(&self) -> &'static str {
        "Starts a new closure, representing its argument\nmost operations applied to arg are added to the closure instead of executed. A few operations will execute the closure on some input, possibly repeatedly."
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("arg")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Nullary(self)
    }
}

impl NullaryExecutor for Arg {

    fn exec(&self) -> Values {
        todo!()
    }
}