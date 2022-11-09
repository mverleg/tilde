use ::std::env::current_exe;

pub use self::parse::parse;
use crate::op::Op;
use crate::op::Prog;
use crate::tilde_log;
use crate::TildeRes;

//TODO @mark: remove unused modules
// mod alphabet;
// mod letter;
mod parse;
// mod word;
mod compact;
mod ops;
