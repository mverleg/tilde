use crate::exec::{BinaryExecutor, Executor, UnaryExecutor};
use crate::exec::stack::Stack;
use crate::op::Op;
use crate::Value;
use crate::Values;

pub fn dispatch_op(stack: &mut impl Stack, op: &Op) -> Values {
    let ret = match op.as_executor() {
        Executor::Nullary(exec) => exec.exec(),
        Executor::Unary(exec) => {
            let top = stack.pop();
            dispatch_unary(exec, top)
        },
        Executor::Binary(exec) => {
            let top = stack.pop();
            let deep = stack.pop();
            dispatch_binary(exec, top, deep)
        }
        Executor::BinaryOpaque => todo!(),
        Executor::TernaryOpaque => todo!(),
    };
    ret
}

pub fn dispatch_unary(exec: &dyn UnaryExecutor, top: Option<Value>) -> Values {
    match top {
        Some(Value::Num(top)) => exec.exec_n(top),
        Some(Value::Txt(top)) => exec.exec_t(top),
        Some(Value::Arr(top)) => exec.exec_a(top),
        Some(Value::Func(top)) => exec.exec_f(top),
        None => exec.exec_empty(),
    }
}

pub fn dispatch_binary(exec: &dyn BinaryExecutor, top: Option<Value>, deep: Option<Value>) -> Values {
    match (deep, top) {
        (Some(Value::Num(deep)), Some(Value::Num(top))) => exec.exec_nn(deep, top),
        (Some(Value::Num(deep)), Some(Value::Txt(top))) => exec.exec_nt(deep, top),
        (Some(Value::Num(deep)), Some(Value::Arr(top))) => exec.exec_na(deep, top),
        (Some(Value::Num(deep)), Some(Value::Func(top))) => exec.exec_nf(deep, top),
        (Some(Value::Txt(deep)), Some(Value::Num(top))) => exec.exec_tn(deep, top),
        (Some(Value::Txt(deep)), Some(Value::Txt(top))) => exec.exec_tt(deep, top),
        (Some(Value::Txt(deep)), Some(Value::Arr(top))) => exec.exec_ta(deep, top),
        (Some(Value::Txt(deep)), Some(Value::Func(top))) => exec.exec_tf(deep, top),
        (Some(Value::Arr(deep)), Some(Value::Num(top))) => exec.exec_an(deep, top),
        (Some(Value::Arr(deep)), Some(Value::Txt(top))) => exec.exec_at(deep, top),
        (Some(Value::Arr(deep)), Some(Value::Arr(top))) => exec.exec_aa(deep, top),
        (Some(Value::Arr(deep)), Some(Value::Func(top))) => exec.exec_af(deep, top),
        (Some(Value::Func(deep)), Some(Value::Num(top))) => exec.exec_fn(deep, top),
        (Some(Value::Func(deep)), Some(Value::Txt(top))) => exec.exec_ft(deep, top),
        (Some(Value::Func(deep)), Some(Value::Arr(top))) => exec.exec_fa(deep, top),
        (Some(Value::Func(deep)), Some(Value::Func(top))) => exec.exec_ff(deep, top),
        (None, Some(Value::Num(single))) => exec.exec_single_n(single),
        (None, Some(Value::Txt(single))) => exec.exec_single_t(single),
        (None, Some(Value::Arr(single))) => exec.exec_single_a(single),
        (None, Some(Value::Func(single))) => exec.exec_single_f(single),
        (None, None) => exec.exec_empty(),
        (Some(top), None) => unreachable!(),
    }
}
