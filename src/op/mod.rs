use crate::op::array::Lookup;
use crate::op::array::Last;
pub use crate::op::stack::Drop;
pub use crate::op::stack::Duplicate;

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
mod array;

pub fn all_non_literals() -> [Op; 9] {
    //TODO @mark:
    [
        Op::of(Plus),
        Op::of(Minus),
        Op::of(Mul),
        Op::of(Div),
        Op::of(IntDiv),
        Op::of(Drop),
        Op::of(Duplicate),
        Op::of(Last),
        Op::of(Lookup),
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
