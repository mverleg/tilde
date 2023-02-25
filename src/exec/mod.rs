use crate::compile::Prog;
use crate::TildeRes;
use crate::Value;

pub use self::executor::BinaryExecutor;
pub use self::executor::Executor;
pub use self::executor::NullaryExecutor;

mod executor;

//TODO @mark: remove?
pub fn execute(
    prog: Prog,
    inp: Vec<String>,
) -> TildeRes<Value> {
    let mut i = 0;
    let mut stack = Vec::new();
    while let Some(op) = prog.get(i) {
        let ret = match op.executor() {
            Executor::Nullary(exec) => exec.exec(),
            Executor::Unary => todo!(),
            Executor::Binary(exec) => {
                let top = stack.pop();
                let deep = stack.pop();
                match (deep, top) {
                    (Some(Value::Num(deep)), Some(Value::Num(top))) => exec.exec_nn(deep, top),
                    (Some(Value::Num(deep)), Some(Value::Txt(top))) => exec.exec_nt(deep, top),
                    (Some(Value::Num(deep)), Some(Value::Arr(top))) => exec.exec_na(deep, top),
                    (Some(Value::Txt(deep)), Some(Value::Num(top))) => exec.exec_tn(deep, top),
                    (Some(Value::Txt(deep)), Some(Value::Txt(top))) => exec.exec_tt(deep, top),
                    (Some(Value::Txt(deep)), Some(Value::Arr(top))) => exec.exec_ta(deep, top),
                    (Some(Value::Arr(deep)), Some(Value::Num(top))) => exec.exec_an(deep, top),
                    (Some(Value::Arr(deep)), Some(Value::Txt(top))) => exec.exec_at(deep, top),
                    (Some(Value::Arr(deep)), Some(Value::Arr(top))) => exec.exec_aa(deep, top),
                    (None, Some(top)) => todo!(),
                    (Some(_), None) => unreachable!(),
                    (None, None) => todo!(),
                }
            }
            Executor::BinaryOpaque => todo!(),
            Executor::TernaryOpaque => todo!(),
        };
        stack.extend(ret);
        i += 1;
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
