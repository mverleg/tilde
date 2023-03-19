use crate::compile::Prog;
use crate::op::Op;
use crate::{tilde_log, Values};
use crate::TildeRes;
use crate::Value;

pub use self::executor::BinaryExecutor;
pub use self::executor::Executor;
pub use self::executor::NullaryExecutor;
pub use self::executor::UnaryExecutor;

mod executor;
mod dispatch;

pub fn execute(
    prog: Prog,
    inp: Value,
) -> TildeRes<Value> {
    let mut i = 0;
    let mut stack = Vec::new();
    stack.push(inp);
    while let Some(op) = prog.get(i) {
        tilde_log!("stack before {:?}: {}", op, stack.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(" | "));
        let ret = dispatch_op(|| stack.pop(), op);
        stack.extend(ret);
        i += 1;
    }
    tilde_log!("final stack: {}", stack.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(" | "));
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
