use crate::common::log;
// use crate::exec::mathbool::exec_math1;
// use crate::exec::mathbool::exec_math2;
use crate::op::Op;
use crate::op::Prog;
use crate::TildeRes;

pub use self::data::Array;
pub use self::data::Number;
pub use self::data::Text;
pub use self::data::Value;

mod data;
// mod mathbool;
//TODO @mark: ^

pub fn execute(
    prog: Prog,
    mut inp: Vec<String>,
) -> TildeRes<Vec<Value>> {
    let mut i = 0;
    let mut stack = Vec::new();
    while let Some(op) = prog.get(i) {

        i += 1;
    }
    Ok(vec![stack.pop().unwrap_or_else(|| Value::None)])

    //TODO @mark: TEMPORARY! REMOVE THIS!

    // inp.reverse();
    // let mut stack = Array::single(Array::of(inp));
    // for op in prog.iter() {
    //     let res = match op {
    //         Op::Math1(op) => exec_math1(op, stack.pop()),
    //         Op::Math2(op) => exec_math2(op, stack.pop(), stack.pop()),
    //         Op::Compare(_compare) => todo!(),
    //         Op::Bool2(_bool2) => todo!(),
    //         Op::Value(value) => todo!(),
    //     };
    //     stack.push(res)
    // }
    // log!("stack at end: {:?}", stack);
    // let out = stack.pop();
    // log!("final value: {:?}", out);
    // Ok(vec![out])
}
