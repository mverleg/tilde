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
            Self::Text(_) => "text",
            Self::Number(_) => "nr",

            Self::Neg => "neg",
            Self::Abs => "abs",
            Self::Incr => "incr",
            Self::Decr => "decr",

            Self::Plus => "plus",
            Self::Minus => "minus",
            Self::Mul => "mul",
            Self::Div => "div",
            Self::IntDiv => "int-div",
            Self::Mod => "modulo",

            Self::Eq => "eq",
            Self::Neq => "neq",
            Self::Gt => "gt",
            Self::Gte => "gte",
            Self::Lt => "lt",
            Self::Lte => "lte",

            Self::And => "and",
            Self::Or => "or",
            Self::Nand => "nand",
            Self::Xor => "xor",
            Self::Impl => "impl",
        }
    }
}
