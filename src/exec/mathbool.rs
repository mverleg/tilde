use crate::ast::Math2Op;
use crate::exec::Value;

pub fn exec_math2(op: &Math2Op, left: Value, right: Value) -> Value {
    match op {
        Math2Op::Plus => {}
        Math2Op::Minus => {}
        Math2Op::Mul => {}
        Math2Op::Div => {}
        Math2Op::Mod => {}
    }
}
