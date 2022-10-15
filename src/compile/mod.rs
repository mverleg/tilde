use crate::ast::Prog;
use crate::compile::lex::lex_str;
use crate::compile::parse::build_ast;
use crate::TildeRes;

pub use self::alphabet::ALPHABET;
pub use self::letter::Letter;
pub use self::letter::LetterType;
pub use self::word::Modifiers;
pub use self::word::Word;

//TODO @mverleg: for now, reject duplicate modifiers and enforce order - this way is can be relaxed later without breaking compatibility

mod alphabet;
mod letter;
mod lex;
mod parse;
mod word;

pub fn parse(src: &str) -> TildeRes<Prog> {
    let words: Vec<Word> = lex_str(src)?;
    build_ast(&words)
}
