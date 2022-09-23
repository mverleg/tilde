use crate::ast::typ::Typ;

#[derive(Debug)]
pub enum Math1Op {
    Neg,
    Abs,
    Incr,
    Decr,
}

impl Math1Op {
    pub fn name(&self) -> &str {
        match self {
            Math1Op::Neg => "neg",
            Math1Op::Abs => "abs",
            Math1Op::Incr => "incr",
            Math1Op::Decr => "decr",
        }
    }

    pub fn description(&self, typ: Typ) -> &str {
        todo!();
    }
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

impl Math2Op {
    pub fn name(&self) -> &str {
        match self {
            Math2Op::Plus => "plus",
            Math2Op::Minus => "minus",
            Math2Op::Mul => "mul",
            Math2Op::Div => "div",
            Math2Op::IntDiv => "intdiv",
            Math2Op::Mod => "modulo",
        }
    }

    pub fn description(&self, typ: Typ) -> &str {
        todo!();
    }
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

impl CompareOp {
    pub fn name(&self) -> &str {
        match self {
            CompareOp::Eq => "eq",
            CompareOp::Neq => "neq",
            CompareOp::Gt => "gt",
            CompareOp::Gte => "gte",
            CompareOp::Lt => "lt",
            CompareOp::Lte => "lte",
        }
    }

    pub fn description(&self, typ: Typ) -> &str {
        todo!();
    }
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

impl Bool2Op {
    pub fn name(&self) -> &str {
        match self {
            Bool2Op::And => "and",
            Bool2Op::Or => "or",
            Bool2Op::Xor => "xor",
            Bool2Op::Impl => "impl",
        }
    }

    pub fn description(&self, typ: Typ) -> &str {
        todo!();
    }
}
