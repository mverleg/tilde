use crate::op::Op;

/// Find [Op] by identifeir. Not applicable for literals (text, number).
pub fn lookup_op_name(op_name: &str) -> Option<Op> {
    Some(match op_name {
        "neg" => Op::Neg,
        "abs" => Op::Abs,
        "incr" => Op::Incr,
        "decr" => Op::Decr,

        "plus" => Op::Plus,
        "minus" => Op::Minus,
        "mul" => Op::Mul,
        "div" => Op::Div,
        "int-div" => Op::IntDiv,
        "mod" => Op::Mod,

        "eq" => Op::Eq,
        "neq" => Op::Neq,
        "gt" => Op::Gt,
        "gte" => Op::Gte,
        "lt" => Op::Lt,
        "lte" => Op::Lte,

        "and" => Op::And,
        "or" => Op::Or,
        "nand" => Op::Nand,
        "xor" => Op::Xor,
        "impl" => Op::Impl,

        _ => return None,
    })
}
