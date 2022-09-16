use crate::parse::{TokenGroup, CHARSET, Token, Modifiers};

/// Generate all possible input starters with up to N modifiers.
pub fn gen_inputs(max_modifier: u8) -> Vec<TokenGroup> {
    let mut tokens = vec![];
    let modifiers = CHARSET.iter()
        .filter(|token| token.typ == TokenType::Modifier)
        .collect();
    for token in &CHARSET {
        match token.typ {
            TokenType::VariableOpen => {

            }
            TokenType::FixedOpen => {}
            TokenType::Modifier => {}
        }
    }
    tokens
}

//TODO @mverleg: smallvec might have been efficient here...
pub fn combine_modifiers(current: Vec<Vec<Token>>, all_modifiers: &[Token], depth_remaining: u8) -> Vec<Vec<Token>> {
    if depth_remaining == 0 {
        return current
    }
    for cur in current {
        for modi in modifiers {
            combine_modifiers(cur.clone(), all_modifiers, depth_remaining - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_inputs_mod2() {
        let tokens = gen_inputs(2);
        assert!(tokens.len() > 256);
    }

    #[test]
    fn combine_two_modifiers() {
        let all = vec![];
        let res = combine_modifiers(vec![], &all, 2);
        assert!(res.len(), )
    }
}
