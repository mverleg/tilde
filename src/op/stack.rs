use ::std::any::Any;
use ::std::borrow::Cow;

use crate::{Array, Value};
use crate::compile::GolfWord;
use crate::compile::Letter;
use crate::data::Fork;
use crate::exec::{BinaryOpaqueExecutor, Executor};
use crate::exec::UnaryExecutor;
use crate::Nr;
use crate::op::Op;
use crate::op::OpTyp;
use crate::Text;
use crate::Values;
use crate::values;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Drop;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Rotate3;
//TODO @mark:

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct RevRotate3;
//TODO @mark:

impl Drop {
    pub fn new() -> Op {
        Op::of(Drop)
    }
}

impl OpTyp for Drop {

    fn description(&self) -> &'static str {
        "drop (pop) the top value from the stack"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("drop")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        Some(GolfWord::new1(Letter::Slash))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Unary(self)
    }
}

impl UnaryExecutor for Drop {

    fn exec_n(&self, value: Nr) -> Values {
        // no action, just drop the input
        values![]
    }

    fn exec_t(&self, value: Text) -> Values {
        // no action, just drop the input
        values![]
    }

    fn exec_a(&self, value: Array) -> Values {
        // no action, just drop the input
        values![]
    }

    // fn exec_f(&self, value: Func) -> Values {
    //     // no action, just drop the input
    //     values![]
    //     //TODO @mark: is it correct that this applies eagerly instead of lazily like most func operations?
    // }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Duplicate;

impl Duplicate {
    pub fn new() -> Op {
        Op::of(Duplicate)
    }
}

impl OpTyp for Duplicate {

    fn description(&self) -> &'static str {
        "duplicate the value at the top of the stack"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("dup")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        Some(GolfWord::new1(Letter::Plus))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::Unary(self)
    }
}

impl UnaryExecutor for Duplicate {

    fn exec_n(&self, value: Nr) -> Values {
        // duplicate the input
        values![value, value]
    }

    fn exec_t(&self, value: Text) -> Values {
        // duplicate the input
        values![value.clone(), value]
    }

    fn exec_a(&self, value: Array) -> Values {
        // duplicate the input
        values![value.fork(), value]
    }

    // fn exec_f(&self, value: Func) -> Values {
    //     // duplicate the input
    //     values![value.clone(), value]
    //     //TODO @mark: is it correct that this applies eagerly instead of lazily like most func operations?
    // }

    fn exec_empty(&self) -> Values {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Swap;

impl Swap {
    pub fn new() -> Op {
        Op::of(Swap)
    }
}

impl OpTyp for Swap {

    fn description(&self) -> &'static str {
        "swap top two stack values"
    }

    fn long_code(&self) -> Cow<'static, str> {
        Cow::Borrowed("swap")
    }

    fn golf_code(&self) -> Option<GolfWord> {
        Some(GolfWord::new1(Letter::Plus))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_executor(&self) -> Executor {
        Executor::BinaryOpaque(self)
    }
}

impl BinaryOpaqueExecutor for Swap {

    fn exec_opaque(&self, deep: Value, top: Value) -> Values {
        values![top, deep]
    }

    fn exec_single_opaque(&self, single: Value) -> Values {
        values![single]  // noop
    }

    fn exec_empty(&self) -> Values {
        Values::new()  // noop
    }
}
