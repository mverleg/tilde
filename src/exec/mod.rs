use crate::compile::Prog;
// use crate::exec::mathbool::exec_math1;
// use crate::exec::mathbool::exec_math2;
use crate::op::Op;
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
    inp: Vec<String>,
) -> TildeRes<Value> {
    let mut i = 0;
    let mut stack = Vec::new();
    while let Some(op) = prog.get(i) {
        match op {
            Op::Text(text) => stack.push(Value::Txt(Text::of(text))),
            Op::Number(nr) => stack.push(Value::Num(Number::of(*nr))),
            Op::Neg => todo!(),
            Op::Abs => todo!(),
            Op::Incr => todo!(),
            Op::Decr => todo!(),
            Op::Plus => todo!(),
            Op::Minus => todo!(),
            Op::Mul => todo!(),
            Op::Div => todo!(),
            Op::IntDiv => todo!(),
            Op::Mod => todo!(),
            Op::Eq => todo!(),
            Op::Neq => todo!(),
            Op::Gt => todo!(),
            Op::Gte => todo!(),
            Op::Lt => todo!(),
            Op::Lte => todo!(),
            Op::And => todo!(),
            Op::Or => todo!(),
            Op::Nand => todo!(),
            Op::Xor => todo!(),
            Op::Impl => todo!(),
        }
        i += 1;
    }
    Ok(stack.pop().unwrap_or(Value::None))

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
