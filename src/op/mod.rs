
pub use self::arithmetic::Div;
pub use self::arithmetic::IntDiv;
pub use self::literal::NumberOp;
pub use self::literal::TextOp;
pub use self::typ::Op;
pub use self::typ::OpTyp;

mod typ;
mod literal;
mod arithmetic;

pub fn all_non_literals() -> [Op; 2] {
    //TODO @mark:
    [
        Op::of(Div),
        Op::of(IntDiv),
    ]
}

#[cfg(test)]
mod op_properties {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn long_identifiers_valid() {
        unimplemented!();  //TODO @mark
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
        let mut names = Vec::with_capacity(2 * ops.len() + 8);
        for op in ops {
            names.push(op.long_code().into_owned());
            names.push(op.golf_code().unwrap().to_string());
        }
        let unique_names = names.iter().collect::<HashSet<_>>();
        assert_eq!(names.len(), unique_names.len());
    }
}

//TODO @mark: long and golf not both empty
//TODO @mark: id unique and sequential
//TODO @mark: name unique and identifier-safe
