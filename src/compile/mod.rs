use ::std::env::current_exe;

use crate::op::Op;
use crate::op::Prog;
use crate::TildeRes;

pub use self::parse::parse;

pub use crate::exec::Value;

//TODO @mark: remove unused modules
// mod alphabet;
mod letter;
mod parse;
// mod word;
mod ops;
mod text_literal;
mod var_uint;
