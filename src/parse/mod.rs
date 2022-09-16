use crate::ast::{Bool2Op, CompareOp, Math1Op, Math2Op, Op, Prog, ValueOp};

mod charset;

pub fn parse(source: &str) -> Result<Prog, String> {
    let mut ops = vec![];
    for c in source.chars() {
        ops.push(match c {
            '!' => Op::Math1(Math1Op::Neg),
            '‖' => Op::Math1(Math1Op::Abs),
            '↑' => Op::Math1(Math1Op::Incr),
            '↓' => Op::Math1(Math1Op::Decr),
            '+' => Op::Math2(Math2Op::Plus),
            '-' => Op::Math2(Math2Op::Minus),
            '*' => Op::Math2(Math2Op::Mul),
            '/' => Op::Math2(Math2Op::Div),
            '\\' => Op::Math2(Math2Op::IntDiv),
            '%' => Op::Math2(Math2Op::Mod),
            '=' => Op::Compare(CompareOp::Eq),
            '≠' => Op::Compare(CompareOp::Neq),
            '>' => Op::Compare(CompareOp::Gt),
            '≥' => Op::Compare(CompareOp::Gte),
            '<' => Op::Compare(CompareOp::Lt),
            '≤' => Op::Compare(CompareOp::Lte),
            '&' => Op::Bool2(Bool2Op::And),
            '|' => Op::Bool2(Bool2Op::Or),
            //'a' => Op::Bool2(Bool2Op::Nand),  //TODO @mverleg:
            '⊕' => Op::Bool2(Bool2Op::Xor),
            '→' => Op::Bool2(Bool2Op::Impl),
            '0'..='9' => Op::Value(ValueOp::Number(c.to_digit(10).unwrap() as f64)),
            '←' => unimplemented!(),
            sym => Err(format!("unknown source symbol: {sym}"))?
        })
    }
    Ok(Prog::of(ops))
}
