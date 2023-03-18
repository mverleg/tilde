use ::std::any::Any;
use ::std::borrow::Cow;

use crate::{Array, Func, Nr, Text, Values, values};
use crate::compile::GolfWord;
use crate::exec::{Executor, UnaryExecutor};
use crate::op::{Op, OpTyp};
use crate::op::collection::Split;

#[derive(Debug, Clone, PartialEq)]
pub struct BaseWords;

impl BaseWords {
    pub fn new() -> Op {
        Op::of(BaseWords)
    }
}

impl OpTyp for BaseWords {

    fn description(&self) -> &'static str {
        "split into words, turn the words into base versions (lowercase, remove special symbols)"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("base-words")
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

impl UnaryExecutor for BaseWords {

    fn exec_n(&self, value: Nr) -> Values {
        values![]
    }

    fn exec_t(&self, value: Text) -> Values {
        let orig_words = Split::split_str(value.as_str());
        let mut san_words = Vec::with_capacity(orig_words.len());
        for mut word in orig_words {
            let lc= word.to_lowercase();
            word.clear();
            for ch in lc.chars() {
                if ch.is_alphanumeric() {
                    word.push(ch)
                }
            }
            if ! word.is_empty() {
                san_words.push(word)
            }
        }
        values![Array::of(san_words)]
    }

    fn exec_a(&self, value: Array) -> Values {
        todo!()
    }

    fn exec_f(&self, value: Func) -> Values {
        todo!()
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}