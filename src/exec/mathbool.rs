use crate::ast::{Math1Op, Math2Op};
use crate::exec::{Number, Value};

pub fn exec_math1(op: &Math1Op, subject: Value) -> Value {
    match op {
        Math1Op::Neg => match subject {
            Value::Num(nr) => Value::Num(Number::of(-nr.value())),
            Value::Arr(mut arr) => {
                arr.get_mut().reverse();
                Value::Arr(arr)
            }
            _ => todo!(),
        }
        Math1Op::Abs => todo!(),
        Math1Op::Incr => todo!(),
        Math1Op::Decr => todo!(),
    }
}

pub fn exec_math2(op: &Math2Op, right: Value, left: Value) -> Value {
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
