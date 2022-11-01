use crate::ast::Op;
use crate::ast::Prog;
use crate::common::log;
use crate::exec::mathbool::exec_math1;
use crate::exec::mathbool::exec_math2;
use crate::TildeRes;

pub use self::data::Array;
pub use self::data::Number;
pub use self::data::Text;
pub use self::data::Value;

mod data;
mod mathbool;

pub fn execute(
    prog: Prog,
    mut inp: Vec<String>,
) -> TildeRes<Vec<Value>> {
    inp.reverse();
    let mut stack = Array::single(Array::of(inp));
    for op in prog.iter() {
        let res = match op {
            Op::Math1(op) => exec_math1(op, stack.pop()),
            Op::Math2(op) => exec_math2(op, stack.pop(), stack.pop()),
            Op::Compare(_compare) => todo!(),
            Op::Bool2(_bool2) => todo!(),
            Op::Value(value) => Value::of_op(value),
        };
        stack.push(res)
    }
    log!("stack at end: {:?}", stack);
    let out = stack.pop();
    log!("final value: {:?}", out);
    Ok(vec![out])
}
