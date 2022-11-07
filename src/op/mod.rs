pub use ::std::slice;
use ::std::vec;

pub use self::mathbool::Bool2Op;
pub use self::mathbool::CompareOp;
pub use self::mathbool::Math1Op;
pub use self::mathbool::Math2Op;
pub use self::prog::Prog;
use self::typ::Typ;
pub use self::value::ValueOp;

mod mathbool;
mod prog;
mod typ;
mod value;

#[derive(Debug)]
pub enum Op {
    Value(ValueOp),
    Math1(Math1Op),
    Math2(Math2Op),
    Compare(CompareOp),
    Bool2(Bool2Op),
}

impl Op {
    pub fn name(&self) -> &str {
        match self {
            Op::Math1(op) => op.name(),
            Op::Math2(op) => op.name(),
            Op::Compare(op) => op.name(),
            Op::Bool2(op) => op.name(),
            Op::Value(op) => op.name(),
        }
    }

    pub fn description(
        &self,
        typ: Typ,
    ) -> &str {
        match self {
            Op::Math1(op) => op.description(typ),
            Op::Math2(op) => op.description(typ),
            Op::Compare(op) => op.description(typ),
            Op::Bool2(op) => op.description(typ),
            Op::Value(op) => op.description(typ),
        }
    }
}
