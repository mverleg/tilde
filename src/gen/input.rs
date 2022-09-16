use crate::parse::{Modifiers, TokenGroup, TokenType, TOKENSET};

/// Generate all possible input starters with up to N modifiers.
pub fn gen_inputs(max_modifier: u8) -> Vec<TokenGroup> {
    let mut tokens = vec![];
    let modifiers = generate_modifiers();
    for token in &TOKENSET {
        match token.typ {
            TokenType::VariableOpen => {}
            TokenType::FixedOpen => {}
            TokenType::Modifier => { /* pass */ }
        }
    }
    tokens
}

fn generate_modifiers() {
    let mod_tokens = TOKENSET
        .iter()
        .filter(|token| token.is_modifier())
        .collect::<Vec<_>>();
    let mut modifiers = vec![Modifiers::empty()];
    for (n, modi1) in mod_tokens.iter().enumerate() {
        modifiers.push(Modifiers::single((*modi1).clone()));
        for modi2 in &mod_tokens[n..] {
            modifiers.push(Modifiers::double((*modi1).clone(), (*modi2).clone()).unwrap())
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
}
