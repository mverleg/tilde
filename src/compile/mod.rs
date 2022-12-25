use ::std::env::current_exe;

use crate::op::Op;
use crate::op::Prog;
use crate::tilde_log;
use crate::TildeRes;

pub use self::parse::parse;

//TODO @mark: remove unused modules
// mod alphabet;
mod letter;
mod parse;
// mod word;
mod ops;
mod text_literal;
mod var_uint;
