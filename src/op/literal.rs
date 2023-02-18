use ::std::borrow::Cow;
use ::std::any::Any;

use ::tinyvec::TinyVec;

use crate::common::escape_for_string;
use crate::compile::{encode_str, GolfWord};
use crate::Nr;
use crate::op::{Op, OpTyp};

#[derive(Debug, PartialEq)]
pub struct TextOp(String);

impl OpTyp for TextOp {

    fn name(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn description(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn long_code(&self) -> Cow<str> {
        Cow::Owned(format!("\"{}\"", escape_for_string(&self.0)))
    }

    fn golf_code(&self) -> Option<GolfWord> {
        //TODO @mark: make a version that doesn't allocate in encode_str?
        let mut content = TinyVec::new();
        content.extend(encode_str(&self.0).unwrap());
        //TODO @mark: is this unwrap safe?
        Some(GolfWord::new(content))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_equal(&self, other: &dyn OpTyp) -> bool {
        other.as_any()
            .downcast_ref::<Self>()
            .map_or(false, |other_cast| self == other_cast)
    }
}

impl TextOp {
    pub fn new_pure(txt: impl Into<String>) -> Self {
        TextOp(txt.into())
    }

    pub fn new(txt: impl Into<String>) -> Op {
        Op::of(TextOp::new_pure(txt))
    }
}

#[derive(Debug, PartialEq)]
pub struct NumberOp(Nr);

impl OpTyp for NumberOp {

    fn name(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn description(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn long_code(&self) -> Cow<str> {
        Cow::Owned(format!("{}", self.0))
    }

    fn golf_code(&self) -> Option<GolfWord> {
        todo!()  //TODO @mark:
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_equal(&self, other: &dyn OpTyp) -> bool {
        other.as_any()
            .downcast_ref::<Self>()
            .map_or(false, |other_cast| self == other_cast)
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
