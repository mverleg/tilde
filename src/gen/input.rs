use crate::parse::{TokenGroup, CHARSET};

/// Generate all possible input starters with up to N modifiers.
pub fn gen_inputs(max_modifier: u8) -> Vec<TokenGroup> {
    let mut tokens = vec![];
    for token in CHARSET {
        todo!()
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
