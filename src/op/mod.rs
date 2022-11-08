pub use ::std::slice;
use ::std::vec;

pub use self::ops::Op;
pub use self::prog::Prog;
use self::typ::Typ;

mod ops;
mod prog;
mod typ;
