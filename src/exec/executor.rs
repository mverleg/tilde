use crate::Values;

/// Different types of execution, based on input.
/// Each of these may push any number of outputs.
#[derive(Debug)]
pub enum Executor<N: NullaryExecutor = NoExec> {
    /// Does not consume any stack values
    Nullary(N),
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

pub trait NullaryExecutor {
    fn exec(&self) -> Values;
}

impl NullaryExecutor for NoExec {
    fn exec(&self) -> Values {
        unimplemented!("use a different executor")
    }
}