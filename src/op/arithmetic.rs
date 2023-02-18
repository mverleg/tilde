use ::std::any::Any;
use ::std::borrow::Cow;

use crate::compile::GolfWord;
use crate::op::Op;
use crate::op::OpTyp;

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
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
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
        todo!()
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("int-div")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

