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
    fn exec(self) -> Values;
}

pub trait BinaryExecutor: Debug {
    fn exec_nn(self, left: Nr, right: Nr) -> Values;

    fn exec_nt(self, left: Nr, right: Text) -> Values;

    fn exec_na(self, left: Nr, right: Array) -> Values;

    fn exec_tn(self, left: Text, right: Nr) -> Values;

    fn exec_tt(self, left: Text, right: Text) -> Values;

    fn exec_ta(self, left: Text, right: Array) -> Values;

    fn exec_an(self, left: Array, right: Nr) -> Values;

    fn exec_at(self, left: Array, right: Text) -> Values;

    fn exec_aa(self, left: Array, right: Array) -> Values;
}
