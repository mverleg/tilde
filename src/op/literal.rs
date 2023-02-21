use ::std::any::Any;
use ::std::borrow::Cow;

use ::tinyvec::TinyVec;

use crate::common::escape_for_string;
use crate::compile::{encode_str, GolfWord};
use crate::{Nr, Values, values};
use crate::exec::{Executor, NullaryExecutor};
use crate::op::Op;
use crate::op::OpTyp;

#[derive(Debug, Clone, PartialEq)]
pub struct TextOp(TextExecutor);

impl OpTyp for TextOp {

    fn description(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Owned(format!("\"{}\"", escape_for_string(&self.0.0)))
    }

    fn golf_code(&self) -> Option<GolfWord> {
        //TODO @mark: make a version that doesn't allocate in encode_str?
        let mut content = TinyVec::new();
        content.extend(encode_str(&self.0.0).unwrap());
        //TODO @mark: is this unwrap safe?
        Some(GolfWord::new(content))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn executor<'a>(&'a self) -> Executor {
        Executor::Nullary(&self.0)
    }
}

impl TextOp {
    pub fn new_pure(txt: impl Into<String>) -> Self {
        TextOp(TextExecutor(txt.into()))
    }

    pub fn new(txt: impl Into<String>) -> Op {
        Op::of(TextOp::new_pure(txt))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TextExecutor(String);

impl NullaryExecutor for TextExecutor {
    fn exec(self) -> Values {
        values![self.0]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberOp(Nr);

impl OpTyp for NumberOp {

    fn description(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Owned(format!("{}", self.0))
    }

    fn golf_code(&self) -> Option<GolfWord> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn executor<'a>(&'a self) -> Executor {
        todo!()
    }
}

impl NumberOp {
    pub fn new_pure(nr: impl Into<Nr>) -> Self {
        NumberOp(nr.into())
    }

    pub fn new(nr: impl Into<Nr>) -> Op {
        Op::of(Self::new_pure(nr))
    }
}
