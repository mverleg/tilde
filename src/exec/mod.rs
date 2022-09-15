use crate::ast::{Op, Prog};

pub use self::data::{Array, Number, Text, Value};

mod data;

pub fn execute(prog: Prog, mut inp: Vec<String>) -> Result<Value, String> {
    inp.reverse();
    let mut stack = Array::single(Array::of(inp));
    for op in prog.iter() {
        match op {
            Op::Math1(_math1) => todo!(),
            Op::Math2(_math2) => todo!(),
            Op::Compare(_compare) => todo!(),
            Op::Bool2(_bool2) => todo!(),
            Op::Value(value) => {
                stack.push(Value::of_op(&value))
            },
        }
    }
    Ok(stack.pop().unwrap_or(Value::None))
}
