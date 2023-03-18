use ::std::any::Any;
use ::std::borrow::Cow;

use ::tinyvec::TinyVec;

use crate::common::escape_for_string;
use crate::compile::encode_str;
use crate::compile::GolfWord;
use crate::exec::{Executor, NullaryExecutor};
use crate::Nr;
use crate::op::Op;
use crate::op::OpTyp;
use crate::Text;
use crate::Values;
use crate::values;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextOp(Text);

impl OpTyp for TextOp {
    fn description(&self) -> &'static str {
        todo!()  //TODO @mark:
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Owned(format!("\"{}\"", escape_for_string(self.0.as_str())))
    }

    fn golf_code(&self) -> Option<GolfWord> {
        //TODO @mark: make a version that doesn't allocate in encode_str?
        let mut content = TinyVec::new();
        content.extend(encode_str(self.0.as_str()).unwrap());
        //TODO @mark: is this unwrap safe?
        Some(GolfWord::new(content))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Nullary(self)
    }
}

impl TextOp {
    pub fn new_pure(txt: impl Into<String>) -> Self {
        TextOp(txt.into().into())
    }

    pub fn new(txt: impl Into<String>) -> Op {
        Op::of(TextOp::new_pure(txt))
    }
}

impl NullaryExecutor for TextOp {
    fn exec(&self) -> Values {
        values![self.0.clone()]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn as_executor(&self) -> Executor {
        Executor::Nullary(self)
    }
}

impl NullaryExecutor for NumberOp {
    fn exec(&self) -> Values {
        values![self.0]
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
