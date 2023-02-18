use ::std::borrow::Cow;

use ::tinyvec::ArrayVec;

use crate::common::escape_for_string;
use crate::compile::{encode_str, Letter};
use crate::Nr;
use crate::op::{Op, OpTyp};

#[derive(Debug)]
pub struct Text(String);

impl OpTyp for Text {

    fn name(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn description(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn long_code(&self) -> Option<Cow<str>> {
        Some(Cow::Owned(format!("\"{}\"", escape_for_string(self.0))))
    }

    fn golf_code(&self) -> Option<ArrayVec<Letter>> {
        Some(encode_str(&self.0).unwrap().into())
        //TODO @mark: is this unwrap safe?
    }
}

#[derive(Debug)]
pub struct Number(Nr);

impl OpTyp for Number {

    fn name(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn description(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn long_code(&self) -> Option<Cow<str>> {
        Some(Cow::Owned(format!("{}", self.0)))
    }

    fn golf_code(&self) -> Option<ArrayVec<Letter>> {
        todo!()  //TODO @mark:
    }
}
