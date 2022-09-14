use crate::ast::{Op, Prog};
pub use self::data::{Value, Array, Text, Number};
mod data;

pub fn execute(prog: Prog, mut inp: Vec<String>) -> Result<Value, String> {
    inp.reverse();
    let mut stack = Array::single(Array::of(inp));
    for op in prog.iter() {
        match op {
            Op::Math1(math1) => {}
            Op::Math2(math2) => {}
            Op::Compare(compare) => {}
            Op::Bool2(bool2) => {}
            Op::Value(value) => {}
        }
    }
    if let Some(top) = stack.pop() {
        println!("{:?}", top)
    }
    Ok(())
}
