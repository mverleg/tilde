use ::std::fmt::Debug;

use crate::{Array, Func};
use crate::Nr;
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

    fn exec_f(&self, value: Func, current_op: &Op) -> Values {
        values![Value::Func(value.with_unary(current_op.clone()))]
    }

    /// Fallback for if the stack is empty
    fn exec_empty(&self) -> Values;
}

pub trait BinaryExecutor: OpTyp {

    fn exec_nn(&self, deep: Nr, top: Nr) -> Values;

    fn exec_nt(&self, deep: Nr, top: Text) -> Values;

    fn exec_na(&self, deep: Nr, top: Array) -> Values;

    fn exec_nf(&self, deep: Nr, top: Func, current_op: &Op) -> Values {
        values![Value::Func(top.with_bin_deep(current_op.clone(), Value::Num(deep)))]
    }

    fn exec_tn(&self, deep: Text, top: Nr) -> Values;

    fn exec_tt(&self, deep: Text, top: Text) -> Values;

    fn exec_ta(&self, deep: Text, top: Array) -> Values;

    fn exec_tf(&self, deep: Text, top: Func, current_op: &Op) -> Values {
        values![Value::Func(top.with_bin_deep(current_op.clone(), Value::Txt(deep)))]
    }

    fn exec_an(&self, deep: Array, top: Nr) -> Values;

    fn exec_at(&self, deep: Array, top: Text) -> Values;

    fn exec_aa(&self, deep: Array, top: Array) -> Values;

    fn exec_af(&self, deep: Array, top: Func, current_op: &Op) -> Values {
        values![Value::Func(top.with_bin_deep(current_op.clone(), Value::Arr(deep)))]
    }

    fn exec_fn(&self, deep: Func, top: Nr, current_op: &Op) -> Values {
        values![Value::Func(deep.with_bin_top(current_op.clone(), Value::Num(top)))]
    }

    fn exec_ft(&self, deep: Func, top: Text, current_op: &Op) -> Values {
        values![Value::Func(deep.with_bin_top(current_op.clone(), Value::Txt(top)))]
    }

    fn exec_fa(&self, deep: Func, top: Array, current_op: &Op) -> Values {
        values![Value::Func(deep.with_bin_top(current_op.clone(), Value::Arr(top)))]
    }

    fn exec_ff(&self, deep: Func, top: Func, current_op: &Op) -> Values {
        values![Value::Func(top.with_bin_deep(current_op.clone(), Value::Func(deep)))]
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
