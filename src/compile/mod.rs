use ::std::env::current_exe;

pub use crate::exec::Value;
use crate::op::Op;
use crate::op::Prog;
use crate::TildeRes;

pub use self::parse::parse;
pub use self::text_literal::Closer;
pub use self::text_literal::encode_uint_vec;
pub use self::var_uint::encode_snippet_len_estimate;

//TODO @mark: remove unused modules
// mod alphabet;
mod letter;
mod parse;
// mod word;
mod ops;
mod text_literal;
mod var_uint;
