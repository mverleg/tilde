use crate::compile::Prog;
use crate::{Array, Text, tilde_log};
use crate::TildeRes;
use crate::Value;

pub use self::executor::BinaryExecutor;
pub use self::executor::Executor;
pub use self::executor::NullaryExecutor;
pub use self::executor::UnaryExecutor;

mod executor;

pub fn execute(
    prog: Prog,
    inp: Vec<Value>,
) -> TildeRes<Value> {
    let mut i = 0;
    let mut stack = Vec::new();
    stack.push(Value::Arr(Array::of(inp.into_iter()
        .collect())));
    while let Some(op) = prog.get(i) {
        println!("stack: {}", Value::Arr(Array::of(stack.clone())));  //TODO @mark: TEMPORARY! REMOVE THIS!
        let ret = match op.as_executor() {
            Executor::Nullary(exec) => exec.exec(),
            Executor::Unary(exec) => {
                let top = stack.pop();
                match top {
                    Some(Value::Num(top)) => exec.exec_n(top),
                    Some(Value::Txt(top)) => exec.exec_t(top),
                    Some(Value::Arr(top)) => exec.exec_a(top),
                    None => exec.exec_empty(),
                }
            },
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
                    (None, Some(Value::Num(single))) => exec.exec_single_n(single),
                    (None, Some(Value::Txt(single))) => exec.exec_single_t(single),
                    (None, Some(Value::Arr(single))) => exec.exec_single_a(single),
                    (None, None) => exec.exec_empty(),
                    (Some(top), None) => unreachable!(),
                }
            }
            Executor::BinaryOpaque => todo!(),
            Executor::TernaryOpaque => todo!(),
        };
        stack.extend(ret);
        i += 1;
    }
    Ok(match stack.pop() {
        Some(top) => {
            tilde_log!("execution done, top item out of {} is {:?}", stack.len(), &top);
            top
        },
        None => {
            tilde_log!("execution done, but stack is empty");
            Value::default()
        }
    })
}
