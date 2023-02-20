use crate::compile::Prog;
use crate::TildeRes;
use crate::Value;

pub use self::executor::Executor;
pub use self::executor::NullaryExecutor;

mod executor;

//TODO @mark: remove?
pub fn execute(
    prog: Prog,
    inp: Vec<String>,
) -> TildeRes<Value> {
    let i = 0;
    let mut stack = Vec::new();
    while let Some(op) = prog.get(i) {
        todo!();
        // match op {
        //     OpTyp::Text(text) => stack.push(Value::Txt(Text::of(text))),
        //     OpTyp::Number(nr) => stack.push(Value::Num(Number::of(*nr))),
        //     OpTyp::Neg => todo!(),
        //     OpTyp::Abs => todo!(),
        //     OpTyp::Incr => todo!(),
        //     OpTyp::Decr => todo!(),
        //     OpTyp::Plus => todo!(),
        //     OpTyp::Minus => todo!(),
        //     OpTyp::Mul => todo!(),
        //     OpTyp::Div => todo!(),
        //     OpTyp::IntDiv => todo!(),
        //     OpTyp::Mod => todo!(),
        //     OpTyp::Eq => todo!(),
        //     OpTyp::Neq => todo!(),
        //     OpTyp::Gt => todo!(),
        //     OpTyp::Gte => todo!(),
        //     OpTyp::Lt => todo!(),
        //     OpTyp::Lte => todo!(),
        //     OpTyp::And => todo!(),
        //     OpTyp::Or => todo!(),
        //     OpTyp::Nand => todo!(),
        //     OpTyp::Xor => todo!(),
        //     OpTyp::Impl => todo!(),
        // }
        // i += 1;
    }
    Ok(stack.pop().unwrap_or(Value::default()))

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
