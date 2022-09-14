
#[derive(Debug)]
pub enum Math1Op {
    Neg,
    Abs,
}

#[derive(Debug)]
pub enum Math2Op {
    Plus,
    Minus,
    Mul,
    Div,
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
    Nand,
    Xor,
    Impl,
}
