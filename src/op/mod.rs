use crate::op::arithmetic::Sqrt;
use crate::op::collection::{Count, Flatten};
use crate::op::collection::Last;
use crate::op::collection::Lookup;
use crate::op::collection::Split;
use crate::op::collection::Sum;
use crate::op::collection::Unique;
pub use crate::op::func::Apply;
use crate::op::func::Arg;
use crate::op::sanitize::BaseWords;
pub use crate::op::stack::Drop;
pub use crate::op::stack::Duplicate;
use crate::op::stack::Swap;
use crate::op::text::Concat;

pub use self::arithmetic::Div;
pub use self::arithmetic::IntDiv;
pub use self::arithmetic::Minus;
pub use self::arithmetic::Mul;
pub use self::arithmetic::Plus;
pub use self::literal::NumberOp;
pub use self::literal::TextOp;
pub use self::typ::Op;
pub use self::typ::OpTyp;

mod typ;
mod literal;
mod arithmetic;
mod stack;
mod collection;
mod sanitize;
mod func;
mod text;

pub fn all_non_literals() -> [Op; 20] {
    //TODO @mark:
    [
        Op::of(Plus),
        Op::of(Minus),
        Op::of(Mul),
        Op::of(Div),
        Op::of(Sqrt),
        Op::of(IntDiv),
        Op::of(Drop),
        Op::of(Duplicate),
        Op::of(Last),
        Op::of(Lookup),
        Op::of(Split),
        Op::of(BaseWords),
        Op::of(Unique),
        Op::of(Count),
        Op::of(Arg),
        Op::of(Apply),
        Op::of(Concat),
        Op::of(Sum),
        Op::of(Swap),
        Op::of(Flatten),
    ]
}

#[cfg(test)]
mod op_properties {
    use ::std::collections::HashSet;

    use super::*;

    #[test]
    fn long_identifiers_valid() {
        let options = "abcdefghijklmnopqrstuvwxyz0123456789-".chars().collect::<HashSet<_>>();
        let start_options = "abcdefghijklmnopqrstuvwxyz".chars().collect::<HashSet<_>>();
        let end_options = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect::<HashSet<_>>();
        for op in all_non_literals() {
            let name = op.long_code().to_string().chars().collect::<Vec<_>>();
            assert!(!name.is_empty());
            for chr in &name {
                assert!(options.contains(&name[0]));
            }
            assert!(start_options.contains(&name[0]));
            assert!(end_options.contains(&name[name.len() - 1]));
        }
    }

    #[test]
    fn short_long_unique() {
        let mut ops = all_non_literals().into_iter().collect::<Vec<_>>();
        ops.push(TextOp::new(""));
        ops.push(TextOp::new("Hello world"));
        ops.push(TextOp::new("你好"));
        ops.push(NumberOp::new(0));
        ops.push(NumberOp::new(-1));
        ops.push(NumberOp::new(123_456_789));
        let mut names = HashSet::with_capacity(2 * ops.len() + 8);
        for op in ops {
            eprintln!("names1 = {:?}", names);  //TODO @mark: TEMPORARY! REMOVE THIS!
            assert!(names.insert(op.long_code().into_owned()), "duplicate Op identifier (long): {}", op.long_code());
            if let Some(name) = op.golf_code() {
                eprintln!("names2 = {:?}", names);  //TODO @mark: TEMPORARY! REMOVE THIS!
                assert!(names.insert(name.to_string()), "duplicate Op identifier (short): {}", name);
            }
        }
        //todo!("why is this doing compression?")
    }
}
