pub use crate::exec::Value;

pub use self::letter::Letter;
pub use self::letter::LetterKind;
pub use self::parse::parse;
pub use self::text_literal::Closer;
pub use self::text_literal::encode_str;
pub use self::text_literal::encode_uint_vec;
pub use self::var_uint::encode_snippet_len_estimate;
pub use self::prog::Prog;
pub use self::typ::Typ;

//TODO @mark: remove unused modules
// mod alphabet;
mod letter;
mod parse;
// mod word;
mod ops;
mod text_literal;
mod var_uint;
pub mod prog;
pub mod typ;
