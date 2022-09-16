use crate::parse::{Modifiers, Token, TokenGroup, TOKENSET};

/// Generate all possible input starters with up to N modifiers.
pub fn gen_inputs(max_modifier: u8) -> Vec<TokenGroup> {
    let mut tokens = vec![];
    let mod_tomens = TOKENSET
        .iter()
        .filter(|token| token.is_modifier())
        .collect();
    let moddifiers = vec![Modifiers::new(None, None)];
    for modi1 in mod_tomens {
        for modi2 in mod_tomens {

        }
    }
    for token in &TOKENSET {
        match token.typ {
            TokenType::VariableOpen => {}
            TokenType::FixedOpen => {}
            TokenType::Modifier => {}
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_inputs_mod2() {
        let tokens = gen_inputs(2);
        assert!(tokens.len() > 256);
    }
}
