use ::std::any::Any;
use ::std::borrow::Cow;

use crate::Array;
use crate::compile::GolfWord;
use crate::exec::BinaryExecutor;
use crate::exec::Executor;
use crate::exec::UnaryExecutor;
use crate::Nr;
use crate::op::Op;
use crate::op::OpTyp;
use crate::Text;
use crate::Values;
use crate::values;

#[derive(Debug, Clone, PartialEq)]
pub struct Last;

impl Last {
    pub fn new() -> Op {
        Op::of(Last)
    }
}

impl OpTyp for Last {

    fn description(&self) -> &'static str {
        "last value of a collection"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("last")
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

//TODO @mark: does nothing yet
impl UnaryExecutor for Last {

    fn exec_n(&self, value: Nr) -> Values {
        todo!()
    }

    fn exec_t(&self, value: Text) -> Values {
        todo!()
    }

    fn exec_a(&self, value: Array) -> Values {
        todo!()
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lookup;

impl Lookup {
    pub fn new() -> Op {
        Op::of(Lookup)
    }
}

impl OpTyp for Lookup {

    fn description(&self) -> &'static str {
        "look up by index"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("index")
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

impl BinaryExecutor for Lookup {

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
        values![deep.index(top)]
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
        values![single.index(Nr::zero())]
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}
