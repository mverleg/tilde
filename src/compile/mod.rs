pub use self::alphabet::ALPHABET;
pub use self::letter::Letter;
pub use self::letter::LetterType;
pub use self::word::Modifiers;
pub use self::word::Word;
use crate::compile::parse::build_ast;
use crate::op::Prog;
use crate::TildeRes;

//TODO @mverleg: for now, reject duplicate modifiers and enforce order - this way is can be relaxed later without breaking compatibility

// mod alphabet;
// mod letter;
mod parse;
// mod word;

pub fn parse(src: &str) -> TildeRes<Prog> {
    let mut ops = vec![];
    let mut current_op_name = String::new();
    let mut current_golfed_word = String::new();
    for letter in src.chars() {
        if letter.is_alphanumeric() {
            current_op_name.push(letter);
            if !current_golfed_word.is_empty() {
                ops.push(golfed_word_to_op(&current_golfed_word));
                current_golfed_word.clear()
            }
        } else if alphabet.contains(letter) {
        }
    }
    if !current_golfed_word.is_empty() {
        ops.push(golfed_word_to_op(&current_golfed_word))
    }
    Ok(Prog::of(ops))
}
