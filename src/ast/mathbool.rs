#[derive(Debug)]
pub enum Math1Op {
    Neg,
    Abs,
    Incr,
    Decr,
}

impl Math1Op {
    pub fn name(&self) -> &str {

    }

    pub fn description(&self) -> &str {

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

    }

    pub fn description(&self) -> &str {

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

    }

    pub fn description(&self) -> &str {

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

    }

    pub fn description(&self) -> &str {

    }
}
