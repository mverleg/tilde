use ::std::fmt::Debug;
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
    Binary,
    ///
    BinaryOpaque,
    ///
    TernaryOpaque,
}

#[derive(Debug)]
struct NoExec;

pub trait NullaryExecutor: Debug {
    fn exec(&self) -> Values;
}

impl NullaryExecutor for NoExec {
    fn exec(&self) -> Values {
        unimplemented!("use a different executor")
    }
}