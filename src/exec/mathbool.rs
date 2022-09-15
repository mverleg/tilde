use crate::ast::Math2Op;
use crate::exec::{Number, Value};

pub fn exec_math2(op: &Math2Op, left: Value, right: Value) -> Value {
    match op {
        Math2Op::Plus => match (left, right) {
            (Value::Num(left), Value::Num(right)) => Value::Num(Number::of(left.value() + right.value())),
            _ => todo!(),
        }
        Math2Op::Minus => match (left, right) {
            (Value::Num(left), Value::Num(right)) => Value::Num(Number::of(left.value() - right.value())),
            _ => todo!(),
        },
        Math2Op::Mul => match (left, right) {
            (Value::Num(left), Value::Num(right)) => Value::Num(Number::of(left.value() * right.value())),
            _ => todo!(),
        },
        Math2Op::Div => match (left, right) {
            (Value::Num(left), Value::Num(right)) => Value::Num(Number::of(left.value() / right.value())),
            _ => todo!(),
        },
        Math2Op::IntDiv => match (left, right) {
            (Value::Num(left), Value::Num(right)) => Value::Num(Number::of(((left.value() as i64) / (right.value() as i64)) as f64)),
            _ => todo!(),
        },
        Math2Op::Mod => match (left, right) {
            (Value::Num(left), Value::Num(right)) => Value::Num(Number::of(left.value() % right.value())),
            _ => todo!(),
        },
    }
}
