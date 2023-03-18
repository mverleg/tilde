use ::std::fmt::Debug;

use crate::Array;
use crate::Func;
use crate::Nr;
use crate::op::NumberOp;
use crate::op::Op;
use crate::op::OpTyp;
use crate::Text;
use crate::values;
use crate::Values;

/// Different types of execution, based on input.
/// Each of these may push any number of outputs.
#[derive(Debug)]
pub enum Executor<'a> {
    /// Does not consume any stack values
    Nullary(&'a dyn NullaryExecutor),
    /// Consumes one stack value
    Unary(&'a dyn UnaryExecutor),
    ///
    Binary(&'a dyn BinaryExecutor),
    ///
    BinaryOpaque,
    ///
    TernaryOpaque,
}

pub trait NullaryExecutor: OpTyp {
    fn exec(&self) -> Values;
}

pub trait UnaryExecutor: OpTyp {

    fn exec_n(&self, value: Nr) -> Values;

    fn exec_t(&self, value: Text) -> Values;

    fn exec_a(&self, value: Array) -> Values;

    fn exec_f(&self, value: Func) -> Values;

    /// Fallback for if the stack is empty
    fn exec_empty(&self) -> Values;
}

pub trait BinaryExecutor: OpTyp {

    fn exec_nn(&self, deep: Nr, top: Nr) -> Values;

    fn exec_nt(&self, deep: Nr, top: Text) -> Values;

    fn exec_na(&self, deep: Nr, top: Array) -> Values;

    fn exec_nf(&self, deep: Nr, top: Func, current_op: &Op) -> Values {
        values![top
            .with_op(NumberOp::new(deep))
            .with_op(current_op.clone())]
    }

    fn exec_tn(&self, deep: Text, top: Nr) -> Values;

    fn exec_tt(&self, deep: Text, top: Text) -> Values;

    fn exec_ta(&self, deep: Text, top: Array) -> Values;

    fn exec_tf(&self, deep: Text, top: Func, current_op: &Op) -> Values {
        values![top
            .with_val(Value::Txt(deep))
            .with_op(current_op.clone())]
        //TODO @mark: prevent clones like this (maybe Op COW?)
    }

    fn exec_an(&self, deep: Array, top: Nr) -> Values;

    fn exec_at(&self, deep: Array, top: Text) -> Values;

    fn exec_aa(&self, deep: Array, top: Array) -> Values;

    fn exec_af(&self, deep: Array, top: Func, current_op: &Op) -> Values {
        values![top
            .with_val(Value::Arr(deep))
            .with_op(current_op.clone())]
    }

    // fn exec_fn(&self, deep: Func, top: Nr) -> Values;

    // fn exec_ft(&self, deep: Func, top: Text) -> Values;

    // fn exec_fa(&self, deep: Func, top: Array) -> Values;
    //TODO @mark: ^ x3

    fn exec_ff(&self, deep: Func, top: Func, current_op: &Op) -> Values {
        values![top
            .with_val(Value::Func(deep))
            .with_op(current_op.clone())]
    }

    /// Fallback for if there is only 1 value on the stack and it is a number
    fn exec_single_n(&self, single: Nr) -> Values;

    /// Fallback for if there is only 1 value on the stack and it is a text
    fn exec_single_t(&self, single: Text) -> Values;

    /// Fallback for if there is only 1 value on the stack and it is a array
    fn exec_single_a(&self, single: Array) -> Values;

    /// Fallback for if there is only 1 value on the stack and it is a function
    fn exec_single_f(&self, single: Func, current_op: &Op) -> Values {
        todo!()
    }

    /// Fallback for if the stack is empty
    fn exec_empty(&self) -> Values;
}
