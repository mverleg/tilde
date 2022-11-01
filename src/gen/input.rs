use crate::compile::LetterType;
use crate::compile::Modifiers;
use crate::compile::Word;
use crate::compile::ALPHABET;

/// Generate all possible input starters with up to N modifiers.
pub fn gen_inputs() -> Vec<Word> {
    let mut groups = vec![];
    let modifiers = generate_modifiers();
    for modi in &modifiers {
        if modi.first().is_some() {
            groups.push(Word::JustMod(modi.clone()));
        }
    }
    for letter in &ALPHABET {
        match letter.typ {
            LetterType::Literal => {
                // TODO @mverleg
            },
            LetterType::VariableOpen => {
                for modi in &modifiers {
                    groups.push(Word::Var(letter.clone(), modi.clone()));
                }
            },
            LetterType::FixedOpen => {
                for second in &ALPHABET {
                    for modi in &modifiers {
                        groups.push(Word::Fixed(letter.clone(), second.clone(), modi.clone()));
                    }
                }
            },
            LetterType::Modifier => { /* pass */ },
        }
    }
    groups
}

fn generate_modifiers() -> Vec<Modifiers> {
    let modifier_letters = ALPHABET.iter().filter(|letter| letter.is_modifier()).collect::<Vec<_>>();
    let mut modifiers = vec![Modifiers::empty()];
    for (n, modi1) in modifier_letters.iter().enumerate() {
        modifiers.push(Modifiers::of_single((*modi1).clone()));
        for modi2 in &modifier_letters[..n] {
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
        let words = gen_inputs();
        assert!(words.len() > 256);
    }
}
