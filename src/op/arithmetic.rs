use ::std::any::Any;
use ::std::borrow::Cow;

use crate::compile::GolfWord;
use crate::exec::{BinaryExecutor, Executor};
use crate::op::Op;
use crate::op::OpTyp;
use crate::{Array, Nr, Text, Values, values};

#[derive(Debug, Clone, PartialEq)]
pub struct Div;

impl Div {
    pub fn new() -> Op {
        Op::of(Div)
    }
}

impl OpTyp for Div {

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

    fn executor<'a>(&'a self) -> Executor {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct DivExecutor;

impl BinaryExecutor for DivExecutor {
    fn exec_nn(self, left: Nr, right: Nr) -> Values {
        values![left.div(right)]
    }

    fn exec_nt(self, left: Nr, right: Text) -> Values {
        todo!()
    }

    fn exec_na(self, left: Nr, right: Array) -> Values {
        todo!()
    }

    fn exec_tn(self, left: Text, right: Nr) -> Values {
        todo!()
    }

    fn exec_tt(self, left: Text, right: Text) -> Values {
        todo!()
    }

    fn exec_ta(self, left: Text, right: Array) -> Values {
        todo!()
    }

    fn exec_an(self, left: Array, right: Nr) -> Values {
        todo!()
    }

    fn exec_at(self, left: Array, right: Text) -> Values {
        todo!()
    }

    fn exec_aa(self, left: Array, right: Array) -> Values {
        todo!()
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct IntDiv;

impl IntDiv {
    pub fn new() -> Op {
        Op::of(IntDiv)
    }
}

impl OpTyp for IntDiv {

    fn description(&self) -> &'static str {
        "divide rounding down"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("int-div")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        None  //TODO @mark:
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn executor<'a>(&'a self) -> Executor {
        todo!()
    }
}

