use crate::ast::{Op, Prog};
use crate::common::log;
use crate::exec::mathbool::exec_math2;

pub use self::data::{Array, Number, Text, Value};

mod data;
mod mathbool;

pub fn execute(prog: Prog, mut inp: Vec<String>) -> Result<Value, String> {
    inp.reverse();
    let mut stack = Array::single(Array::of(inp));
    for op in prog.iter() {
        match op {
            Op::Math1(_math1) => todo!(),
            Op::Math2(op) => {
                let left = stack.pop();
                let right = stack.pop();
                stack.push(exec_math2(op, left, right))
            },
            Op::Compare(_compare) => todo!(),
            Op::Bool2(_bool2) => todo!(),
            Op::Value(value) => {
                stack.push(Value::of_op(&value))
            },
        }
    }
    log!("stack at end: {:?}", stack);
    let out = stack.pop();
    log!("final value: {:?}", out);
    Ok(out)
}
