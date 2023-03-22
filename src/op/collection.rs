use ::std::any::Any;
use ::std::borrow::Cow;
use ::std::collections::HashSet;

use crate::Array;
use crate::Text;
use crate::compile::GolfWord;
use crate::exec::dispatch_binary;
use crate::exec::BinaryExecutor;
use crate::exec::Executor;
use crate::exec::UnaryExecutor;
use crate::Nr;
use crate::op::Op;
use crate::op::Plus;
use crate::op::OpTyp;
use crate::Value;
use crate::Values;
use crate::values;

// thread_local! {
//     static SPLIT_RE: LazyCell<Regex> = LazyCell::new(|| Regex::new("\\s+").unwrap());
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lookup;

impl Lookup {
    pub fn new() -> Op {
        Op::of(Lookup)
    }
}

impl OpTyp for Lookup {

    fn description(&self) -> &'static str {
        "look up by index or key"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("lookup")
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Split;

impl Split {
    pub fn new() -> Op {
        Op::of(Split)
    }

    pub fn split_str(text: &str) -> Vec<String> {
        text
            .split_whitespace()
            .map(|slice| slice.to_owned())
            .collect::<Vec<_>>()
    }
}

impl OpTyp for Split {

    fn description(&self) -> &'static str {
        "split a string or array by a separator"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("split")
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

impl UnaryExecutor for Split {

    fn exec_n(&self, value: Nr) -> Values {
        todo!()
    }

    fn exec_t(&self, value: Text) -> Values {
        let words = Split::split_str(value.as_str());
        values![Array::of(words)]
    }

    fn exec_a(&self, value: Array) -> Values {
        todo!()
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Unique;

impl Unique {
    pub fn new() -> Op {
        Op::of(Unique)
    }

    pub fn split_str(text: &str) -> Vec<String> {
        text
            .split_whitespace()
            .map(|slice| slice.to_owned())
            .collect::<Vec<_>>()
    }
}

impl OpTyp for Unique {

    fn description(&self) -> &'static str {
        "remove any duplicates, keeping the first occurence"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("unique")
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

impl UnaryExecutor for Unique {

    fn exec_n(&self, value: Nr) -> Values {
        todo!()
    }

    fn exec_t(&self, value: Text) -> Values {
        todo!()
    }

    fn exec_a(&self, value: Array) -> Values {
        let mut seen = HashSet::with_capacity(value.len());
        let mut result = Vec::with_capacity(value.len());
        for val in value.into_iter() {
            if seen.insert(val.fork()) {
                //TODO @mark: ^ remove clone
                result.push(val)
            }
        }
        values![Value::Arr(Array::of(result))]
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Count;

impl Count {
    pub fn new() -> Op {
        Op::of(Count)
    }

    pub fn split_str(text: &str) -> Vec<String> {
        text
            .split_whitespace()
            .map(|slice| slice.to_owned())
            .collect::<Vec<_>>()
    }
}

impl OpTyp for Count {

    fn description(&self) -> &'static str {
        "count number of items (length)"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("count")
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

impl UnaryExecutor for Count {

    fn exec_n(&self, value: Nr) -> Values {
        todo!()
    }

    fn exec_t(&self, value: Text) -> Values {
        let len = value.len() as f64;
        values![Value::Num(Nr::new(len))]
    }

    fn exec_a(&self, value: Array) -> Values {
        let len = value.len() as f64;
        values![Value::Num(Nr::new(len))]
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sum;

impl Sum {
    pub fn new() -> Op {
        Op::of(Sum)
    }
}

impl OpTyp for Sum {

    fn description(&self) -> &'static str {
        "sum all the parts of value"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("sum")
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

impl UnaryExecutor for Sum {
    fn exec_n(&self, value: Nr) -> Values {
        todo!()
    }

    fn exec_t(&self, value: Text) -> Values {
        todo!()
    }

    fn exec_a(&self, value: Array) -> Values {
        let mut total = Value::Txt(Text::empty());
        for item in value {
            let mut res = dispatch_binary(&Plus, Some(total), Some(item));
            total = res.pop().expect("plus did not yield result");
            assert!(res.is_empty());
        }
        values![total]
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}
