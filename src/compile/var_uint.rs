use ::std::fmt;
use ::std::fmt::Formatter;
use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::compile::var_uint::DecodeError::TooLarge;
use crate::op::Op;

const STRING_NOMOD_OPENERS: [Letter; 10] = [Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon, Number];
const STRING_WITHMOD_OPENERS: [Letter; 14] = [Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon, Hat, Exclamation, Question, Hash, Tilde];
const STRING_FOLLOWERS: [Letter; 16] = [Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon, Hat, Exclamation, Question, Hash, Tilde, Number, Text];
const STRING_NOMOD_OPENERS_VALUES: [UINT; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, UINT::MAX, UINT::MAX, UINT::MAX, UINT::MAX, UINT::MAX, 9, UINT::MAX];
const STRING_WITHMOD_OPENERS_VALUES: [UINT; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, UINT::MAX, UINT::MAX];
const STRING_FOLLOWER_VALUES: [UINT; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

/// Encode a positive integer using variable length.
/// * Each letter represents a half-byte. They do not follow default order.
/// * The first letter is not allowed to be Text, to distinguish end of text after previous number.
/// * The first letter is not allowed to be a modifier, because such modifiers apply to the string itself.
/// * The value of letters depends on position and is given by order in the constant arrays (possibly different from `Letter.nr()`).
/// * Any letter in the upper half marks the end of a number, and the real value is minus half the length.
/// * The first three (opener+2) letters can mark the end of the number, then every 2nd of next two letters, then every 3rd for next two, etc.
///   Positions that cannot mark the end of a number, can use the full range of letters instead of just half.
pub fn encode_uint_no_modifier_at_start(nr: UINT) -> Vec<Letter> {
    encode_uint_with_openers(nr, &STRING_NOMOD_OPENERS)
}

/// Similar to [encode_uint_no_modifier_at_start], except:
/// * The first letter is allowed to be a modifier. It is not allowed to be text token.
/// * The first letter is also not allowed to be number token. The length has to be even, so we can
///   freely nominate one, which we pick to be number token, for use in closing special string literals.
pub fn encode_uint_allow_modifiers(nr: UINT) -> Vec<Letter> {
    encode_uint_with_openers(nr, &STRING_WITHMOD_OPENERS)
}

#[inline]
fn encode_uint_with_openers(
    nr: UINT,
    openers: &[Letter],
) -> Vec<Letter> {
    let mut letters = vec![];
    let opener_n = (openers.len() / 2) as UINT;
    if nr < opener_n {
        letters.push(openers[(nr + opener_n) as usize]);
    } else {
        letters.push(openers[(nr % opener_n) as usize]);
    }
    let mut non_close_letter_cnt_doubled = 0;
    let follow_2n = STRING_FOLLOWERS.len() as UINT;
    let follow_1n = follow_2n / 2;
    debug_assert!(follow_1n <= 8 && (follow_1n as usize) < usize::MAX);
    let mut rem = nr / opener_n;
    while rem > 0 {
        rem = rem.saturating_sub(1);
        for i in 0..(non_close_letter_cnt_doubled / 2) {
            letters.push(STRING_FOLLOWERS[(rem % follow_2n) as usize]);
            rem = rem / follow_2n;
        }
        let pos = if rem < follow_1n { rem + follow_1n } else { rem % follow_1n };
        letters.push(STRING_FOLLOWERS[pos as usize]);
        rem = rem / follow_1n;
        non_close_letter_cnt_doubled += 1;
    }
    return letters;
}

/// Inverse of [encode_pos_int_static_width_avoid_modifiers].
pub fn decode_uint_no_modifier_at_start(letters: &[Letter]) -> Result<DecodedPositiveNumber, DecodeError> {
    if letters.is_empty() {
        return Err(DecodeError::NoInput);
    }
    let opener = &letters[0];
    if Letter::modifiers().contains(opener) {
        return Err(DecodeError::StarsWithModifier);
    }
    decode_uint_with_openers(letters, &STRING_NOMOD_OPENERS, &STRING_NOMOD_OPENERS_VALUES)
}

/// Inverse of [encode_pos_int_static_width_avoid_modifiers].
pub fn decode_uint_allow_modifiers(letters: &[Letter]) -> Result<DecodedPositiveNumber, DecodeError> {
    if letters.is_empty() {
        return Err(DecodeError::NoInput);
    }
    decode_uint_with_openers(letters, &STRING_WITHMOD_OPENERS, &STRING_WITHMOD_OPENERS_VALUES)
}

#[inline]
fn decode_uint_with_openers(
    letters: &[Letter],
    openers: &[Letter],
    opener_values: &[UINT],
) -> Result<DecodedPositiveNumber, DecodeError> {
    let opener = &letters
        .iter()
        .next()
        .expect("empty input when decoding int, should be checked before calling");
    if let Letter::Text = opener {
        return Err(DecodeError::TextNode);
    }
    let value = opener_values[opener.nr() as usize];
    if value >= 16 {
        return Err(DecodeError::UnexpectedNode);
    }
    let open_n = (openers.len() / 2) as UINT;
    if value >= open_n {
        return Ok(DecodedPositiveNumber { end_index: 0, number: value - open_n });
    };
    let mut nr = value;
    let mut multiplier = open_n;
    let follow_2n = STRING_FOLLOWERS.len() as UINT;
    let follow_1n = follow_2n / 2;
    let mut letter_i: usize = 1;
    let mut non_close_letter_cnt_doubled = 0;
    let mut block_addition = 1;
    while letter_i.saturating_add(non_close_letter_cnt_doubled / 2) < letters.len() {
        for _block_offset in 0..(non_close_letter_cnt_doubled / 2) {
            let value = STRING_FOLLOWER_VALUES[letters[letter_i].nr() as usize].saturating_add(block_addition);
            block_addition = 0;
            let addition = multiplier
                .checked_mul(value)
                .ok_or(DecodeError::TooLarge)?;
            nr = nr
                .checked_add(addition)
                .ok_or(DecodeError::TooLarge)?;
            multiplier = multiplier
                .checked_mul(follow_2n)
                .ok_or(DecodeError::TooLarge)?;
            letter_i += 1;
        }
        let value = STRING_FOLLOWER_VALUES[letters[letter_i].nr() as usize];
        if value >= follow_1n {
            let addition = multiplier
                .checked_mul(value.saturating_add(block_addition) - follow_1n)
                .ok_or(DecodeError::TooLarge)?;
            nr = nr
                .checked_add(addition)
                .ok_or(DecodeError::TooLarge)?;
            return Ok(DecodedPositiveNumber { end_index: letter_i, number: nr });
        }
        let addition = multiplier
            .checked_mul(value.saturating_add(block_addition))
            .ok_or(DecodeError::TooLarge)?;
        nr = nr
            .checked_add(addition)
            .ok_or(DecodeError::TooLarge)?;
        multiplier = multiplier
            .checked_mul(follow_1n)
            .ok_or(DecodeError::TooLarge)?;

        non_close_letter_cnt_doubled += 1;
        block_addition = 1;
        letter_i += 1;
    }
    Err(DecodeError::NoEndMarker)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecodedPositiveNumber {
    pub end_index: usize,
    pub number: UINT,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecodeError {
    NoInput,
    TextNode,
    UnexpectedNode,
    StarsWithModifier,
    TooLarge,
    NoEndMarker,
}

impl fmt::Display for DecodeError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DecodeError::NoInput => "number to decode is empty",
                DecodeError::TextNode => "encountered unexpected text node while decoding number",
                DecodeError::UnexpectedNode => "encountered unexpected (non-text) node while decoding number",
                DecodeError::StarsWithModifier => "number to decode starts with a modifier, which is not allowed here",
                DecodeError::TooLarge => "number to decode is too large",
                DecodeError::NoEndMarker => "unexpected end while decoding number; last letter should be marked",
            }
        )
    }
}

#[cfg(test)]
mod constants_in_sync {
    use super::*;
    use crate::compile::letter::LetterKind;

    fn select_letters(predicate: impl Fn(&Letter) -> bool) -> Vec<Letter> {
        let mut allowed: Vec<Letter> = Letter::iter()
            .filter(|letter| !Letter::modifiers().contains(letter))
            .filter(|letter| letter != &Text && letter != &Number)
            .filter(&predicate)
            .collect();
        // Put Text and modifiers in the second half (if allowed) to keep the order more stable.
        for letter in Letter::modifiers()
            .into_iter()
            .chain(Some(Number).into_iter())
            .chain(Some(Text).into_iter())
        {
            if predicate(&letter) {
                allowed.push(letter);
            }
        }
        if allowed.len() % 2 != 0 {
            allowed.pop();
        }
        assert_eq!(allowed.len() % 2, 0, "should be an even number of letters, because any odd tail will be ignored");
        assert!(allowed.len() >= 2, "must allow at least two letters, since half are used to indicate closing");
        allowed
    }

    fn reverse_letter_values(letters: &[Letter]) -> Vec<UINT> {
        let mut expected = vec![UINT::MAX; 16];
        for (value, letter) in letters.iter().enumerate() {
            expected[letter.nr() as usize] = value.try_into().unwrap();
        }
        assert_eq!(
            expected
                .iter()
                .filter(|val| **val < 16)
                .count(),
            letters.len()
        );
        expected
    }

    /// The first letter cannot start with a modifier, because those can be used to modify
    /// the start of the string itself, thus not being part of the (first) letter.
    /// It can also not contain a Text letter, because that may signal the end of the
    /// series of numbers (i.e. after the last number, not to be confused with the next number).
    #[test]
    fn openers_nomod() {
        assert_eq!(STRING_NOMOD_OPENERS, select_letters(|letter| letter.kind() != LetterKind::Modifier && letter != &Letter::Text).as_slice());
    }

    #[test]
    fn openers_withmod() {
        assert_eq!(STRING_WITHMOD_OPENERS, select_letters(|letter| letter.kind() != LetterKind::Literal).as_slice());
    }

    /// After the start of the number, everything is allowed - encoutnering any of the second
    /// half of letters will signal the end of the number.
    #[test]
    fn followers() {
        assert_eq!(STRING_FOLLOWERS, select_letters(|_letter| true).as_slice());
    }

    #[test]
    fn reverse_openers_nomod() {
        let expected = reverse_letter_values(&STRING_NOMOD_OPENERS);
        assert_eq!(STRING_NOMOD_OPENERS_VALUES, expected.as_slice());
    }

    #[test]
    fn reverse_openers_withmod() {
        let expected = reverse_letter_values(&STRING_WITHMOD_OPENERS);
        assert_eq!(STRING_WITHMOD_OPENERS_VALUES, expected.as_slice());
    }

    #[test]
    fn reverse_followers() {
        let expected = reverse_letter_values(&STRING_FOLLOWERS);
        assert_eq!(STRING_FOLLOWER_VALUES, expected.as_slice());
    }
}

#[cfg(test)]
mod test_util {
    use ::std::collections::HashSet;

    use super::*;
    use crate::compile::letter::LetterKind;
    use crate::compile::var_uint::DecodeError::TextNode;

    pub fn encoding_to_str_for_debug(letters: &[Letter]) -> String {
        let enc = letters
            .iter()
            .map(|l| l.symbol().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let digits = letters
            .iter()
            .map(|l| STRING_FOLLOWER_VALUES[l.nr() as usize].to_string())
            .collect::<Vec<_>>()
            .join(" ");
        format!("{enc}   {digits}")
    }
}

#[cfg(test)]
mod dynamic_width_common_without_modifiers {
    use ::std::collections::HashSet;

    use crate::compile::letter::LetterKind;
    use crate::compile::var_uint::DecodeError::TextNode;

    #[macro_use]
    use super::*;
    use super::test_util::*;

    pub fn encode(nr: UINT) -> Vec<Letter> {
        encode_uint_no_modifier_at_start(nr)
    }

    pub fn decode(letters: &[Letter]) -> DecodedPositiveNumber {
        decode_uint_no_modifier_at_start(letters).unwrap()
    }

    #[test]
    fn encoding_examples() {
        assert_eq!(encode(0), vec![Slash]);
        assert_eq!(encode(4), vec![Number]);
        assert_eq!(encode(5), vec![Io, Colon]);
        assert_eq!(encode(44), vec![Asterisk, Text]);
        assert_eq!(encode(45), vec![Io, Io, Colon]);
        assert_eq!(encode(364), vec![Asterisk, Bracket, Text]);
        assert_eq!(encode(365), vec![Io, Io, Io, Io, Colon]);
        assert_eq!(encode(41_324), vec![Asterisk, Bracket, Bracket, Text, Text]);
        assert_eq!(encode(41_325), vec![Io, Io, Io, Io, Io, Io, Colon]);
        assert_eq!(encode(5_284_204), vec![Asterisk, Bracket, Bracket, Text, Bracket, Text, Text]);
        assert_eq!(encode(5_284_205), vec![Io, Io, Io, Io, Io, Io, Io, Io, Io, Colon]);
        assert_eq!(encode(10_742_702_444), vec![Asterisk, Bracket, Bracket, Text, Bracket, Text, Bracket, Text, Text, Text]);
        assert_eq!(encode(10_742_702_445), vec![Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Colon]);
        assert_eq!(encode(18_873_338_202_050), vec![Io, Seq, More, Colon, Plus, Hat, Asterisk, Exclamation, Question, Slash, Hash, Tilde, Number]);
    }

    #[test]
    fn decoding_examples() {
        assert_eq!(decode(&[Slash]).number, 0);
        assert_eq!(decode(&[Number]).number, 4);
        assert_eq!(decode(&[Io, Colon]).number, 5);
        assert_eq!(decode(&[Asterisk, Text]).number, 44);
        assert_eq!(decode(&[Io, Io, Colon]).number, 45);
        assert_eq!(decode(&[Asterisk, Bracket, Text]).number, 364);
        assert_eq!(decode(&[Io, Io, Io, Io, Colon]).number, 365);
        assert_eq!(decode(&[Asterisk, Bracket, Bracket, Text, Text]).number, 41_324);
        assert_eq!(decode(&[Io, Io, Io, Io, Io, Io, Colon]).number, 41_325);
        assert_eq!(decode(&[Asterisk, Bracket, Bracket, Text, Bracket, Text, Text]).number, 5_284_204);
        assert_eq!(decode(&[Io, Io, Io, Io, Io, Io, Io, Io, Io, Colon]).number, 5_284_205);
        assert_eq!(decode(&[Asterisk, Bracket, Bracket, Text, Bracket, Text, Bracket, Text, Text, Text]).number, 10_742_702_444);
        assert_eq!(decode(&[Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Colon]).number, 10_742_702_445);
        assert_eq!(decode(&[Io, Seq, More, Colon, Plus, Hat, Asterisk, Exclamation, Question, Slash, Hash, Tilde, Number]).number, 18_873_338_202_050);
    }

    #[test]
    fn decode_end_index() {
        assert_eq!(decode(&[Slash, Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket]).end_index, 0);
        assert_eq!(decode(&[Io, Colon]).end_index, 1);
        assert_eq!(decode(&[Asterisk, Bracket, Bracket, Text, Text, Asterisk, Hash]).end_index, 4);
        assert_eq!(decode(&[Io, Io, Plus, Io, Plus, Io, Io, Io, Plus, Colon, Hash]).end_index, 9);
        assert_eq!(decode(&[Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Colon, Hash]).end_index, 12);
    }

    #[test]
    fn positive_int_starting_with_modifiers() {
        let letters = &[Letter::modifiers()[0], Asterisk];
        let dec = decode_uint_no_modifier_at_start(letters);
        assert_eq!(dec, Err(DecodeError::StarsWithModifier));
    }

    common_tests!(encode_uint_no_modifier_at_start, decode_uint_no_modifier_at_start);
}

#[cfg(test)]
mod dynamic_width_common_allow_modifiers {
    use ::std::collections::HashSet;

    use crate::compile::letter::LetterKind;
    use crate::compile::var_uint::DecodeError::TextNode;

    #[macro_use]
    use super::*;
    use super::test_util::*;

    pub fn encode(nr: UINT) -> Vec<Letter> {
        encode_uint_allow_modifiers(nr)
    }

    pub fn decode(letters: &[Letter]) -> DecodedPositiveNumber {
        decode_uint_allow_modifiers(letters).unwrap()
    }

    #[test]
    fn encoding_examples() {
        assert_eq!(encode(0), vec![Bracket]);
        assert_eq!(encode(6), vec![Tilde]);
        assert_eq!(encode(7), vec![Io, Colon]);
        assert_eq!(encode(62), vec![Right, Text]);
        assert_eq!(encode(63), vec![Io, Io, Colon]);
        assert_eq!(encode(510), vec![Right, Bracket, Text]);
        assert_eq!(encode(511), vec![Io, Io, Io, Io, Colon]);
        assert_eq!(encode(57_854), vec![Right, Bracket, Bracket, Text, Text]);
        assert_eq!(encode(57_855), vec![Io, Io, Io, Io, Io, Io, Colon]);
        assert_eq!(encode(7_397_886), vec![Right, Bracket, Bracket, Text, Bracket, Text, Text]);
        assert_eq!(encode(7_397_887), vec![Io, Io, Io, Io, Io, Io, Io, Io, Io, Colon]);
        assert_eq!(encode(15_039_783_422), vec![Right, Bracket, Bracket, Text, Bracket, Text, Bracket, Text, Text, Text]);
        assert_eq!(encode(15_039_783_423), vec![Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Colon]);
        assert_eq!(encode(26_422_676_238_522), vec![Asterisk, Seq, More, Text, Plus, Hat, Bracket, Exclamation, Question, Slash, Hash, Tilde, Number]);
    }

    #[test]
    fn decoding_examples() {
        assert_eq!(decode(&[Bracket]).number, 0);
        assert_eq!(decode(&[Tilde]).number, 6);
        assert_eq!(decode(&[Io, Colon]).number, 7);
        assert_eq!(decode(&[Right, Text]).number, 62);
        assert_eq!(decode(&[Io, Io, Colon]).number, 63);
        assert_eq!(decode(&[Right, Bracket, Text]).number, 510);
        assert_eq!(decode(&[Io, Io, Io, Io, Colon]).number, 511);
        assert_eq!(decode(&[Right, Bracket, Bracket, Text, Text]).number, 57_854);
        assert_eq!(decode(&[Io, Io, Io, Io, Io, Io, Colon]).number, 57_855);
        assert_eq!(decode(&[Right, Bracket, Bracket, Text, Bracket, Text, Text]).number, 7_397_886);
        assert_eq!(decode(&[Io, Io, Io, Io, Io, Io, Io, Io, Io, Colon]).number, 7_397_887);
        assert_eq!(decode(&[Right, Bracket, Bracket, Text, Bracket, Text, Bracket, Text, Text, Text]).number, 15_039_783_422);
        assert_eq!(decode(&[Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Colon]).number, 15_039_783_423);
        assert_eq!(decode(&[Asterisk, Seq, More, Text, Plus, Hat, Bracket, Exclamation, Question, Slash, Hash, Tilde, Number]).number, 26_422_676_238_522);
    }

    #[test]
    fn decode_end_index() {
        assert_eq!(decode(&[Bracket, Seq]).end_index, 0);
        assert_eq!(decode(&[Io, Colon, Right, Seq]).end_index, 1);
        assert_eq!(decode(&[Asterisk, Hash, More]).end_index, 1);
        assert_eq!(decode(&[Right, Text, Asterisk, Exclamation]).end_index, 1);
        assert_eq!(decode(&[Right, Bracket, Bracket, Text, Text, Exclamation, Number]).end_index, 4);
        assert_eq!(decode(&[Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Io, Colon, Io, Io, Io]).end_index, 12);
    }

    #[test]
    fn positive_int_starting_with_modifiers() {
        let letters = &[Letter::modifiers()[0], Asterisk];
        let dec = decode_uint_allow_modifiers(letters);
        assert!(dec.is_ok());
    }

    common_tests!(encode_uint_allow_modifiers, decode_uint_allow_modifiers);
}

macro_rules! common_tests {
    ($encode: ident, $decode: ident) => {
        #[test]
        fn all_encodings_unique() {
            let n = 50_000;
            let mut seen = HashSet::with_capacity(n as usize);
            for i in 0..n {
                let enc = $encode(i);
                assert!(enc.len() != 4 && enc.len() != 6, "nr {i} has impossible length {}", enc.len());
                assert!(seen.insert(enc.clone()), "nr {i} has same encoding as an earlier nr");
                assert_eq!(i, $decode(&enc).unwrap().number, "decode not same for nr {i}");
            }
        }

        #[test]
        fn encode_and_decode_samples() {
            let nrs = (0..100).chain((7..60).map(|n| (2 as UINT).pow(n) - n as UINT));
            for nr in nrs {
                let enc = $encode(nr);
                let dec = $decode(&enc).unwrap_or_else(|err| panic!("failed to decode {}, err {}", nr, err));
                assert_eq!(nr, dec.number);
            }
        }

        #[test]
        fn positive_int_avoid_modifiers_empty_input() {
            let dec = $decode(&[]);
            assert_eq!(dec, Err(DecodeError::NoInput));
        }

        #[test]
        fn positive_int_non_terminated_number() {
            let dec = $decode(&[Io]);
            assert_eq!(dec, Err(DecodeError::NoEndMarker));
            let dec = $decode(&[Io, Io, Io]);
            assert_eq!(dec, Err(DecodeError::NoEndMarker));
        }

        #[test]
        fn positive_int_decode_with_text_opener() {
            let dec = $decode(&[Text, Hash]);
            assert_eq!(dec, Err(DecodeError::TextNode));
        }

        #[test]
        /// Because the number of letters is even, and Text is not allowed at the end, (at least) one other letter
        /// should also not be allowed. This checks that that is handled gracefully.
        fn positive_int_decode_with_unused_letter() {
            for list in [STRING_NOMOD_OPENERS.as_slice(), STRING_FOLLOWERS.as_slice()] {
                let mut unused = HashSet::new();
                for letter in Letter::iter() {
                    unused.insert(letter);
                }
                unused.remove(&Text);
                for letter in Letter::modifiers() {
                    unused.remove(&letter);
                }
                for letter in list {
                    unused.remove(&letter);
                }
                if unused.is_empty() {
                    eprintln!("no unused letters")
                }
                for letter in unused {
                    pub(self) use common_tests;
                    let dec = $decode(&[Number, letter, Hash]);
                    assert_eq!(dec, Err(DecodeError::UnexpectedNode), "list len: {}, letter: {}", list.len(), letter.symbol());
                }
            }
        }

        #[test]
        fn positive_int_overflow_in_decode() {
            let dec = $decode(&[Io; 100]);
            assert_eq!(dec, Err(DecodeError::TooLarge));
        }

        #[test]
        #[ignore]
        fn print_all_encodings_for_debug() {
            //TODO @mverleg: remove?
            for i in 0..=10_000 {
                let letters = $encode(i);
                println!("{i}  {}", encoding_to_str_for_debug(&letters))
            }
            panic!()
        }
    };
}

pub(self) use common_tests;

use crate::UINT;
