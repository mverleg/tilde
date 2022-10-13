use crate::parse::{Modifiers, TokenGroup, TokenType, TOKENSET};

/// Generate all possible input starters with up to N modifiers.
pub fn gen_inputs() -> Vec<TokenGroup> {
    let mut groups = vec![];
    let modifiers = generate_modifiers();
    for modi in &modifiers {
        if modi.first().is_some() {
            groups.push(TokenGroup::JustMod(modi.clone()));
        }
    }
    for token in &TOKENSET {
        match token.typ {
            TokenType::Literal => {
                // TODO @mverleg
            }
            TokenType::VariableOpen => {
                for modi in &modifiers {
                    groups.push(TokenGroup::Var(token.clone(), modi.clone()));
                }
            }
            TokenType::FixedOpen => {
                for second in &TOKENSET {
                    for modi in &modifiers {
                        groups.push(TokenGroup::Fixed(
                            token.clone(),
                            second.clone(),
                            modi.clone(),
                        ));
                    }
                }
            }
            TokenType::Modifier => { /* pass */ }
        }
    }
    groups
}

fn generate_modifiers() -> Vec<Modifiers> {
    let mod_tokens = TOKENSET
        .iter()
        .filter(|token| token.is_modifier())
        .collect::<Vec<_>>();
    let mut modifiers = vec![Modifiers::empty()];
    for (n, modi1) in mod_tokens.iter().enumerate() {
        modifiers.push(Modifiers::of_single((*modi1).clone()));
        for modi2 in &mod_tokens[..n] {
            modifiers.push(Modifiers::of_double((*modi1).clone(), (*modi2).clone()).unwrap())
        }
    }
    modifiers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_inputs_mod2() {
        let tokens = gen_inputs();
        assert!(tokens.len() > 256);
    }
}
