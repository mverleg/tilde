
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


//TODO @mark: long and golf not both empty
//TODO @mark: id unique and sequential
//TODO @mark: name unique and identifier-safe
