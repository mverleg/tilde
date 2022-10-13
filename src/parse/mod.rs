use crate::ast::Prog;
use crate::parse::build_ast::build_ast;
use crate::parse::tokenize::tokenize;
use crate::TildeRes;

pub use self::token::Token;
pub use self::token::TokenType;
pub use self::tokengroup::Modifiers;
pub use self::tokengroup::TokenGroup;
pub use self::tokenset::TOKENSET;

//TODO @mverleg: for now, reject duplicate modifiers and enforce order - this way is can be relaxed later without breaking compatibility

mod build_ast;
mod token;
mod tokengroup;
mod tokenize;
mod tokenset;

pub fn parse(src: &str) -> TildeRes<Prog> {
    let tokens: Vec<TokenGroup> = tokenize(src)?;
    build_ast(&tokens)
}
