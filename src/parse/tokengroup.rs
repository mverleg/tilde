use crate::parse::Token;

//TODO @mverleg: this is only suitable for general context for now

#[derive(Debug)]
pub struct Modifiers {}

#[derive(Debug)]
pub enum TokenGroup {
    Var(Token, Modifiers),
    Fixed(Token, Token, Modifiers),
    JustMod(Modifiers),
}
