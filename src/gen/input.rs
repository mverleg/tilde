/// Generate all possible input starters with up to N modifiers.
pub fn gen_inputs(max_modifier: u8) {
    for t in TOKENS {}
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
