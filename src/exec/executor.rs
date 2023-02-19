use crate::Values;

/// Different types of execution, based on input.
/// Each of these may push any number of outputs.
pub enum Executor<'a> {
    /// Does not consume any stack values
    Nullary(dyn NullableExecutor<'a>),
    /// Consumes one stack value
    Unary,
    ///
    Binary,
    ///
    BinaryOpaque,
    ///
    TernaryOpaque,
}

pub trait NullableExecutor<'a> {
    fn exec(&'a self) -> Values;
}
