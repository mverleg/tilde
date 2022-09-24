use crate::ast::{Bool2Op, CompareOp, Math1Op, Math2Op, Op, Prog, ValueOp};
use crate::TildeRes;

pub use self::token::Token;
pub use self::token::TokenType;
pub use self::tokengroup::Modifiers;
pub use self::tokengroup::TokenGroup;
pub use self::tokenset::TOKENSET;

//TODO @mverleg: for now, reject duplicate modifiers and enforce order - this way is can be relaxed later without breaking compatibility

mod token;
mod tokengroup;
mod tokenset;
mod tokenize;
mod link_ops;
