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
            assert!(!names.insert(op.long_code().into_owned()));
            if let Some(name) = op.golf_code() {
                assert!(!names.insert(name.to_string()));
            }
        }
        let unique_names = names.iter().collect::<HashSet<_>>();
        assert_eq!(names.len(), unique_names.len());
    }
}
