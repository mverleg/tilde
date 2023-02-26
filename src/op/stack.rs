use ::std::any::Any;
use ::std::borrow::Cow;

use crate::Array;
use crate::compile::GolfWord;
use crate::exec::{Executor, UnaryExecutor};
use crate::Nr;
use crate::op::Op;
use crate::op::OpTyp;
use crate::Text;
use crate::Values;
use crate::values;

#[derive(Debug, Clone, PartialEq)]
pub struct Drop;

#[derive(Debug, Clone, PartialEq)]
pub struct Duplicate;

#[derive(Debug, Clone, PartialEq)]
pub struct Clockwise3;

#[derive(Debug, Clone, PartialEq)]
pub struct CounterClockwise3;

impl Drop {
    pub fn new() -> Op {
        Op::of(Drop)
    }
}

impl OpTyp for Drop {

    fn description(&self) -> &'static str {
        "divide without rounding"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("div")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        None  //TODO @mark:
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Unary(self)
    }
}

impl UnaryExecutor for Drop {

    fn exec_n(&self, value: Nr) -> Values {
        // no action, just drop the input
        values![]
    }

    fn exec_t(&self, value: Text) -> Values {
        // no action, just drop the input
        values![]
    }

    fn exec_a(&self, value: Array) -> Values {
        // no action, just drop the input
        values![]
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}
