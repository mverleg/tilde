use crate::ast::Math2Op;
use crate::exec::{Number, Value};

pub fn exec_math2(op: &Math2Op, left: Value, right: Value) -> Value {
    match op {
        Math2Op::Plus => match (left, right) {
            (Value::Num(left), Value::Num(right)) => Value::Num(Number::of(left.value() + right.value())),
            _ => todo!(),
        }
        Math2Op::Minus => todo!(),
        Math2Op::Mul => todo!(),
        Math2Op::Div => todo!(),
        Math2Op::Mod => todo!(),
    }
}
