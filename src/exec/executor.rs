use ::std::fmt::Debug;
use ::std::any::Any;
use ::std::borrow::Cow;

use crate::Array;
use crate::compile::GolfWord;
use crate::Nr;
use crate::op::OpTyp;
use crate::Text;
use crate::Values;

/// Different types of execution, based on input.
/// Each of these may push any number of outputs.
#[derive(Debug)]
pub enum Executor<'a> {
    /// Does not consume any stack values
    Nullary(&'a dyn NullaryExecutor),
    //TODO @mark: prevent boxing here ^
    /// Consumes one stack value
    Unary,
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

pub trait BinaryExecutor: OpTyp {
    fn exec_nn(&self, deep: Nr, top: Nr) -> Values;

    fn exec_nt(&self, deep: Nr, top: Text) -> Values;

    fn exec_na(&self, deep: Nr, top: Array) -> Values;

    fn exec_tn(&self, deep: Text, top: Nr) -> Values;

    fn exec_tt(&self, deep: Text, top: Text) -> Values;

    fn exec_ta(&self, deep: Text, top: Array) -> Values;

    fn exec_an(&self, deep: Array, top: Nr) -> Values;

    fn exec_at(&self, deep: Array, top: Text) -> Values;

    fn exec_aa(&self, deep: Array, top: Array) -> Values;

    /// Fallback for if there is only 1 value on the stack and it is a number
    fn exec_single_n(&self, single: Nr) -> Values;

    /// Fallback for if there is only 1 value on the stack and it is a text
    fn exec_single_t(&self, single: Text) -> Values;

    /// Fallback for if there is only 1 value on the stack and it is a array
    fn exec_single_a(&self, single: Array) -> Values;

    /// Fallback for if the stack is empty
    fn exec_empty(&self) -> Values;
}
