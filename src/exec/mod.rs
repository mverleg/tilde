use crate::compile::Prog;
use crate::exec::dispatch::dispatch_op;
use crate::exec::stack::new_large_stack;
use crate::tilde_log;
use crate::TildeRes;
use crate::Value;

pub use self::executor::BinaryExecutor;
pub use self::executor::Executor;
pub use self::executor::NullaryExecutor;
pub use self::executor::UnaryExecutor;

mod executor;
mod dispatch;
mod stack;

pub fn execute(
    prog: Prog,
    inp: Value,
) -> TildeRes<Value> {
    let mut i = 0;
    let mut stack = new_large_stack();
    stack.push(inp);
    while let Some(op) = prog.get(i) {
        tilde_log!("stack before {:?}: {}", op, stack.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(" | "));
        let ret = dispatch_op(&mut stack, op);
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
