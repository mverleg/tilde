use ::std::any::Any;
use ::std::borrow::Cow;
use ::std::cell::LazyCell;

use crate::{Array, Nr, Text, Values, values};
use crate::compile::{GolfWord, Letter};
use crate::exec::{Executor, UnaryExecutor};
use crate::op::{Op, OpTyp};

thread_local! {
    static SPLIT_RE: LazyCell<Regex> = LazyCell::new(|| Regex::new("\\s+").unwrap());
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseWords;

impl BaseWords {
    pub fn new() -> Op {
        Op::of(BaseWords)
    }
}

impl OpTyp for BaseWords {

    fn description(&self) -> &'static str {
        "drop (pop) the top value from the stack"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("drop")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Unary(self)
    }
}

impl UnaryExecutor for BaseWords {

    fn exec_n(&self, value: Nr) -> Values {
        values![]
    }

    fn exec_t(&self, value: Text) -> Values {
        value.as_str().split()
    }

    fn exec_a(&self, value: Array) -> Values {
        values![]
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}