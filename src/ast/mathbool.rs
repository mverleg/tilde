#[derive(Debug)]
pub enum Math1Op {
    Neg,
    Abs,
    Incr,
    Decr,
}

#[derive(Debug)]
pub enum Math2Op {
    Plus,
    Minus,
    Mul,
    Div,
    IntDiv,
    Mod,
}

#[derive(Debug)]
pub enum CompareOp {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Debug)]
pub enum Bool2Op {
    And,
    Or,
    //Nand,
    //TODO @mverleg: ^ ?
    Xor,
    Impl,
}
