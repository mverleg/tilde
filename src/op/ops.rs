use crate::op::typ::Typ;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    // Value
    Text(String),
    Number(f64),
    // TODO @mverleg: ^ change to something exact

    // Unary
    Neg,
    Abs,
    Incr,
    Decr,

    // Binary math
    Plus,
    Minus,
    Mul,
    Div,
    IntDiv,
    Mod,

    // Comparison (binary)
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,

    // Binary logic
    And,
    Or,
    Nand,
    Xor,
    Impl,
}

impl Op {
    pub fn name(&self) -> &str {
        match self {
            Math1Op::Neg => "neg",
            Math1Op::Abs => "abs",
            Math1Op::Incr => "incr",
            Math1Op::Decr => "decr",

            Math2Op::Plus => "plus",
            Math2Op::Minus => "minus",
            Math2Op::Mul => "mul",
            Math2Op::Div => "div",
            Math2Op::IntDiv => "intdiv",
            Math2Op::Mod => "modulo",

            CompareOp::Eq => "eq",
            CompareOp::Neq => "neq",
            CompareOp::Gt => "gt",
            CompareOp::Gte => "gte",
            CompareOp::Lt => "lt",
            CompareOp::Lte => "lte",

            Bool2Op::And => "and",
            Bool2Op::Or => "or",
            Bool2Op::Nand => "nand",
            Bool2Op::Xor => "xor",
            Bool2Op::Impl => "impl",
        }
    }
}
