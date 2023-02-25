use ::std::fmt::Debug;

use crate::Array;
use crate::Nr;
use crate::Text;
use crate::Values;

/// Different types of execution, based on input.
/// Each of these may push any number of outputs.
#[derive(Debug)]
pub enum Executor<'a> {
    /// Does not consume any stack values
    Nullary(&'a dyn NullaryExecutor),
    /// Consumes one stack value
    Unary,
    ///
    Binary(&'a dyn BinaryExecutor),
    ///
    BinaryOpaque,
    ///
    TernaryOpaque,
}

pub trait NullaryExecutor: Debug {
    fn exec(&self) -> Values;
}

pub trait BinaryExecutor: Debug {
    fn exec_nn(&self, deep: Nr, top: Nr) -> Values;

    fn exec_nt(&self, deep: Nr, top: Text) -> Values;

    fn exec_na(&self, deep: Nr, top: Array) -> Values;

    fn exec_tn(&self, deep: Text, top: Nr) -> Values;

    fn exec_tt(&self, deep: Text, top: Text) -> Values;

    fn exec_ta(&self, deep: Text, top: Array) -> Values;

    fn exec_an(&self, deep: Array, top: Nr) -> Values;

    fn exec_at(&self, deep: Array, top: Text) -> Values;

    fn exec_aa(&self, deep: Array, top: Array) -> Values;
}
