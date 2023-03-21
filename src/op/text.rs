use ::std::any::Any;
use ::std::borrow::Cow;

use crate::Text;
use crate::Nr;
use crate::Array;
use crate::Values;
use crate::compile::GolfWord;
use crate::exec::BinaryExecutor;
use crate::exec::Executor;
use crate::op::Op;
use crate::op::OpTyp;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Concat;

impl Concat {
    pub fn new() -> Op {
        Op::of(Concat)
    }
}

impl OpTyp for Concat {

    fn description(&self) -> &'static str {
        "concatenate two things, putting them after each other and joining into one"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("concat")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Binary(self)
    }
}

impl BinaryExecutor for Concat {
    fn exec_nn(&self, deep: Nr, top: Nr) -> Values {
        todo!()
    }

    fn exec_nt(&self, deep: Nr, top: Text) -> Values {
        todo!()
    }

    fn exec_na(&self, deep: Nr, top: Array) -> Values {
        todo!()
    }

    fn exec_tn(&self, deep: Text, top: Nr) -> Values {
        todo!()
    }

    fn exec_tt(&self, deep: Text, top: Text) -> Values {
        todo!()
    }

    fn exec_ta(&self, deep: Text, top: Array) -> Values {
        todo!()
    }

    fn exec_an(&self, deep: Array, top: Nr) -> Values {
        todo!()
    }

    fn exec_at(&self, deep: Array, top: Text) -> Values {
        todo!()
    }

    fn exec_aa(&self, deep: Array, top: Array) -> Values {
        todo!()
    }

    fn exec_single_n(&self, single: Nr) -> Values {
        todo!()
    }

    fn exec_single_t(&self, single: Text) -> Values {
        todo!()
    }

    fn exec_single_a(&self, single: Array) -> Values {
        todo!()
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}