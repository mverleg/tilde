use ::std::borrow::Cow;

use crate::common::escape_for_string;
use crate::compile::encode_str;
use crate::compile::GolfWord;
use crate::Nr;
use crate::op::{Op, OpTyp};

#[derive(Debug)]
pub struct TextOp(String);

impl OpTyp for TextOp {

    fn name(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn description(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn long_code(&self) -> Cow<str> {
        Cow::Owned(format!("\"{}\"", escape_for_string(self.0)))
    }

    fn golf_code(&self) -> Option<GolfWord> {
        Some(encode_str(&self.0).unwrap().into())
        //TODO @mark: is this unwrap safe?
    }
}

impl TextOp {
    pub fn new_pure(txt: impl Into<String>) -> Self {
        TextOp(txt)
    }

    pub fn new(txt: impl Into<String>) -> Op {
        Op(TextOp(txt))
    }
}

#[derive(Debug)]
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
}

impl NumberOp {
    pub fn new_pure(nr: impl Into<Nr>) -> Self {
        NumberOp(nr.into())
    }

    pub fn new(nr: impl Into<Nr>) -> Op {
        Op(Self::new_pure(nr))
    }
}
