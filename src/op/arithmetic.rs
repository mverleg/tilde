use ::std::any::Any;
use ::std::borrow::Cow;

use crate::Array;
use crate::compile::GolfWord;
use crate::data::Fork;
use crate::exec::BinaryExecutor;
use crate::exec::Executor;
use crate::exec::UnaryExecutor;
use crate::Nr;
use crate::op::Op;
use crate::op::OpTyp;
use crate::op::text::Concat;
use crate::Text;
use crate::Value;
use crate::Values;
use crate::values;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Plus;

impl Plus {
    pub fn new() -> Op {
        Op::of(Plus)
    }
}

impl OpTyp for Plus {

    fn description(&self) -> &'static str {
        "addition (plus)"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("plus")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        None  //TODO @mark:
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Binary(self)
    }
}

impl BinaryExecutor for Plus {
    fn exec_nn(&self, deep: Nr, top: Nr) -> Values {
        values![deep.plus(top)]
    }

    fn exec_nt(&self, deep: Nr, top: Text) -> Values {
        match top.as_str().parse::<Nr>() {
            Ok(nr) => self.exec_nn(deep, nr),
            Err(_) => Concat.exec_nt(deep, top),
        }
    }

    fn exec_na(&self, deep: Nr, top: Array) -> Values {
        todo!()
    }

    fn exec_tn(&self, deep: Text, top: Nr) -> Values {
        match deep.as_str().parse::<Nr>() {
            Ok(nr) => self.exec_nn(nr, top),
            Err(_) => Concat.exec_tn(deep, top),
        }
    }

    fn exec_tt(&self, deep: Text, top: Text) -> Values {
        todo!()
    }

    fn exec_ta(&self, deep: Text, top: Array) -> Values {
        todo!()
    }

    fn exec_an(&self, deep: Array, top: Nr) -> Values {
        let mut new = Vec::new();
        for item in deep.iter() {
            //TODO @mark: this flattens results if there are more than 1, is that correct? or should they be nested arrays?
            new.extend(match item {
                Value::Num(item) => self.exec_nn(item.fork(), top),
                Value::Txt(item) => self.exec_tn(item.fork(), top),
                Value::Arr(item) => self.exec_an(item.fork(), top),
                Value::Func(item) => self.exec_fn(item.fork(), top),
            })
        }
        values![Array::of(new)]
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
        todo!()
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Minus;

impl Minus {
    pub fn new() -> Op {
        Op::of(Minus)
    }
}

impl OpTyp for Minus {

    fn description(&self) -> &'static str {
        "subtraction (minus)"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("minus")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        None  //TODO @mark:
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Binary(self)
    }
}

impl BinaryExecutor for Minus {
    fn exec_nn(&self, deep: Nr, top: Nr) -> Values {
        values![deep.minus(top)]
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
        todo!()
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
        todo!()
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mul;

impl Mul {
    pub fn new() -> Op {
        Op::of(Minus)
    }
}

impl OpTyp for Mul {

    fn description(&self) -> &'static str {
        "multiplication (times)"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("mul")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        None  //TODO @mark:
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Binary(self)
    }
}

impl BinaryExecutor for Mul {
    fn exec_nn(&self, deep: Nr, top: Nr) -> Values {
        values![deep.mul(top)]
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
        todo!()
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
        todo!()
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn as_executor(&self) -> Executor {
        Executor::Binary(self)
    }
}

impl BinaryExecutor for Div {
    fn exec_nn(&self, deep: Nr, top: Nr) -> Values {
        values![deep.div(top)]
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
        todo!()
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
        todo!()
    }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn as_executor(&self) -> Executor {
        todo!()  //TODO @mark: TEMPORARY! REMOVE THIS!
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sqrt;

impl Sqrt {
    pub fn new() -> Op {
        Op::of(Sqrt)
    }
}

impl OpTyp for Sqrt {

    fn description(&self) -> &'static str {
        "square root"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("sqrt")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        None  //TODO @mark:
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Unary(self)
    }
}

impl UnaryExecutor for Sqrt {
    fn exec_n(&self, value: Nr) -> Values {
        values![Value::Num(value.abs_sqrt())]
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
