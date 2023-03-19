use ::std::any::Any;
use ::std::borrow::Cow;

use crate::{Array, Value};
use crate::compile::GolfWord;
use crate::exec::BinaryExecutor;
use crate::exec::Executor;
use crate::exec::NullaryExecutor;
use crate::Func;
use crate::Nr;
use crate::op::Op;
use crate::op::OpTyp;
use crate::Text;
use crate::Values;
use crate::values;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arg;

impl Arg {
    pub fn new() -> Op {
        Op::of(Arg)
    }
}

impl OpTyp for Arg {

    fn description(&self) -> &'static str {
        "Starts a new closure, representing its argument\nMost operations applied to arg are added to the closure instead of executed. A few operations, like apply, will execute the closure on some input, possibly repeatedly."
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
        values![Value::Func(Func::new())]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Apply;

impl Apply {
    pub fn new() -> Op {
        Op::of(Apply)
    }
}

impl OpTyp for Apply {

    fn description(&self) -> &'static str {
        "Applies a closure to an argument."
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("apply")
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

impl BinaryExecutor for Apply {

    fn exec_nn(&self, deep: Nr, top: Nr) -> Values {
        todo!()
    }

    fn exec_nt(&self, deep: Nr, top: Text) -> Values {
        todo!()
    }

    fn exec_na(&self, deep: Nr, top: Array) -> Values {
        todo!()
    }

    fn exec_nf(&self, deep: Nr, top: Func, _: &Op) -> Values {
        let stack = top.run_on_single(Value::Num(deep));
        Values::from(stack)
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

    fn exec_tf(&self, deep: Text, top: Func, _: &Op) -> Values {
        let stack = top.run_on_single(Value::Txt(deep));
        Values::from(stack)
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

    fn exec_af(&self, deep: Array, top: Func, _: &Op) -> Values {
        let stack = top.run_on_single(Value::Arr(deep));
        Values::from(stack)
    }

    fn exec_fn(&self, deep: Func, top: Nr, _: &Op) -> Values {
        todo!()
    }

    fn exec_ft(&self, deep: Func, top: Text, _: &Op) -> Values {
        todo!()
    }

    fn exec_fa(&self, deep: Func, top: Array, _: &Op) -> Values {
        todo!()
    }

    fn exec_ff(&self, deep: Func, top: Func, _: &Op) -> Values {
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

    fn exec_single_f(&self, single: Func, _: &Op) -> Values {
        todo!()
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}
