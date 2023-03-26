use crate::compile::Prog;
use crate::op::Apply;
use crate::op::Op;
use crate::tilde_log;
use crate::TildeRes;
use crate::Value;

pub use self::dispatch::dispatch_binary;
pub use self::dispatch::dispatch_op;
pub use self::dispatch::dispatch_unary;
pub use self::executor::BinaryExecutor;
pub use self::executor::Executor;
pub use self::executor::NullaryExecutor;
pub use self::executor::UnaryExecutor;
pub use self::stack::new_large_stack;
pub use self::stack::new_small_stack;
pub use self::stack::Stack;

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
        tilde_log!("stack before {:?}: {}", op, stack.as_debug_str());
        let ret = dispatch_op(&mut stack, op);
        stack.push_all(ret);
        i += 1;
    }
    if let Some(Value::Func(func)) = stack.peek() {
        tilde_log!("all ops done, but top of stack is function, adding apply op");
        dbg!(&stack);  //TODO @mark: TEMPORARY! REMOVE THIS!
        let ret = dispatch_op(&mut stack, &Op::of(Apply));
        stack.push_all(ret);
    }
    tilde_log!("final stack: {}", stack.as_debug_str());
    Ok(match stack.pop() {
        Some(top) => {
            tilde_log!("execution done, top item out of {} is {:?}", stack.size(), &top);
            top
        },
        None => {
            tilde_log!("execution done, but stack is empty");
            Value::default()
        }
    })
}
