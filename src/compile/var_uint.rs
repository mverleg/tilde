use ::std::fmt;
use ::std::fmt::Formatter;
use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::compile::var_uint::DecodeError::TooLarge;
use crate::op::Op;

const STRING_OPENERS: [Letter; 4] = [Number, Io, Bracket, Colon];
const STRING_FOLLOWERS: [Letter; 4] = [Number, Io, Bracket, Colon];
const STRING_OPENERS_VALUES: [u64; 16] =
    [0, u64::MAX, 1, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, 2, 3, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX];
const STRING_FOLLOWER_VALUES: [u64; 16] =
    [0, u64::MAX, 1, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, 2, 3, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX];
// const STRING_OPENERS_VALUES: [u64; 16] = [0, u64::MAX, 1, 2, 3, 4, 5, 6, 7, 8, 9, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX];
// const STRING_FOLLOWER_VALUES: [u64; 16] = [0, 15, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
//TODO @mark: TEMPORARY! REMOVE THIS! ^^^^

/// Encode a postive integer using variable length.
/// * Each letter represents a half-byte. They do not follow default order.
/// * The first letter is not allowed to be Text, to distinguish end of text after previous number.
/// * The first letter is now allowed to be a modifier, because such modifiers apply to the string itself.
/// * The value of lettere depends on position and is given by order in the constant arrays (different from `Letter.nr()`).
/// * Any letter in the upper half marks the end of a number, and the real value is minus half the length.
/// * The first three (opener+2) letters can close the number, then every 2nd of next two letters, then every 3rd for next two, etc.
///   Letters that cannot close the number, can use the full range of values instead of just half.
pub fn encode_uint_no_modifier_at_start(nr: u64) -> Vec<Letter> {
    let mut letters = vec![];
    let opener_n = (STRING_OPENERS.len() / 2) as u64;
    if nr < opener_n {
        letters.push(STRING_OPENERS[(nr + opener_n) as usize]);
    } else {
        letters.push(STRING_OPENERS[(nr % opener_n) as usize]);
        print!("init:{} ", nr % opener_n); //TODO @mark: TEMPORARY! REMOVE THIS!
    }
    let mut non_close_letter_cnt_doubled = 0;
    let follow_2n = STRING_FOLLOWERS.len() as u64;
    let follow_1n = follow_2n / 2;
    debug_assert!(follow_1n <= 8 && (follow_1n as usize) < usize::MAX);
    let mut rem = nr / opener_n;
    print!("rem.{rem} "); //TODO @mark: TEMPORARY! REMOVE THIS!
    while rem > 0 {
        print!("| ");
        if rem.saturating_sub(1) / 2 != rem / 2 {
            print!("* ")
        }; //TODO @mark: TEMPORARY! REMOVE THIS!
        rem = rem.saturating_sub(1);
        print!("rem-{rem} "); //TODO @mark: TEMPORARY! REMOVE THIS!
        for i in 0..(non_close_letter_cnt_doubled / 2) {
            //println!("{nr} non-close {rem} ({i})"); //TODO @mark: TEMPORARY! REMOVE THIS!
            letters.push(STRING_FOLLOWERS[(rem % follow_2n) as usize]);
            print!("pos:{} ", rem % follow_2n); //TODO @mark: TEMPORARY! REMOVE THIS!
            rem = rem / follow_2n;
            print!("rem:{rem} "); //TODO @mark: TEMPORARY! REMOVE THIS!
        }
        //println!("{nr} potential close {rem}"); //TODO @mark: TEMPORARY! REMOVE THIS!
        let pos = if rem < follow_1n { rem + follow_1n } else { rem % follow_1n };
        print!("pos;{pos} "); //TODO @mark: TEMPORARY! REMOVE THIS!
        letters.push(STRING_FOLLOWERS[pos as usize]);
        rem = rem / follow_1n;
        print!("rem;{rem} "); //TODO @mark: TEMPORARY! REMOVE THIS!
        non_close_letter_cnt_doubled += 1;
    }
    print!("2block:{non_close_letter_cnt_doubled} || "); //TODO @mark: TEMPORARY! REMOVE THIS!
    return letters;
}

/// Inverse of [encode_pos_int_static_width_avoid_modifiers].
pub fn decode_positive_int_static_width_avoid_modifiers(letters: &[Letter]) -> Result<DecodedPositiveNumber, DecodeError> {
    if letters.is_empty() {
        return Err(DecodeError::NoInput);
    }
    print!("len={} ", letters.len()); //TODO @mverleg: TEMPORARY! REMOVE THIS!
    let opener = &letters[0];
    print!("La:{}={} ", opener.symbol(), STRING_OPENERS_VALUES[opener.nr() as usize]); //TODO @mverleg: TEMPORARY! REMOVE THIS!
    if let Letter::Text = opener {
        return Err(DecodeError::TextNode);
    }
    if Letter::modifiers().contains(opener) {
        return Err(DecodeError::StarsWithModifier);
    }
    let value = STRING_OPENERS_VALUES[opener.nr() as usize];
    if value >= 16 {
        return Err(DecodeError::UnexpectedNode);
    }
    let open_n = (STRING_OPENERS.len() / 2) as u64;
    if value >= open_n {
        return Ok(DecodedPositiveNumber { end_index: 0, number: value - open_n });
    };
    let mut nr = value;
    print!("nr:{nr} "); //TODO @mverleg: TEMPORARY! REMOVE THIS!
    let mut multiplier = open_n;
    let follow_2n = STRING_FOLLOWERS.len() as u64;
    let follow_1n = follow_2n / 2;
    let mut block_i = 1;
    let mut non_close_letter_cnt_doubled = 0;
    print!("{}<{}? ", block_i + (non_close_letter_cnt_doubled / 2), letters.len()); //TODO @mark: TEMPORARY! REMOVE THIS!
    let mut block_addition = 1;
    //TODO @mverleg: use saturating versions here?
    while block_i + (non_close_letter_cnt_doubled / 2) < letters.len() {
        for block_offset in 0..(non_close_letter_cnt_doubled / 2) {
            let value = STRING_FOLLOWER_VALUES[letters[block_i + block_offset].nr() as usize].saturating_add(block_addition);
            block_addition = 0;
            print!("blok Lb:{}={} ", letters[block_i + block_offset].symbol(), value); //TODO @mverleg: TEMPORARY! REMOVE THIS!
            let addition = multiplier
                .checked_mul(value)
                .ok_or(DecodeError::TooLarge)?;
            nr = nr
                .checked_add(addition)
                .ok_or(DecodeError::TooLarge)?;
            print!("nr:{nr} "); //TODO @mverleg: TEMPORARY! REMOVE THIS!
            multiplier = multiplier
                .checked_mul(follow_2n)
                .ok_or(DecodeError::TooLarge)?;
        }
        let letter_i = block_i + (non_close_letter_cnt_doubled / 2);
        let value = STRING_FOLLOWER_VALUES[letters[letter_i].nr() as usize];
        print!("Lc[{letter_i}]:{}={}+{} ", letters[letter_i].symbol(), STRING_FOLLOWER_VALUES[letters[letter_i].nr() as usize], block_addition); //TODO @mverleg: TEMPORARY! REMOVE THIS!
        if value >= follow_1n {
            print!("end {value}>={follow_1n} last={letter_i} "); //TODO @mverleg: TEMPORARY! REMOVE THIS!
            let addition = multiplier
                .checked_mul(value.saturating_add(block_addition) - follow_1n)
                .ok_or(DecodeError::TooLarge)?;
            nr = nr
                .checked_add(addition)
                .ok_or(DecodeError::TooLarge)?;
            print!("nr:{nr} "); //TODO @mverleg: TEMPORARY! REMOVE THIS!
            print!("|| "); //TODO @mverleg: TEMPORARY! REMOVE THIS!
            return Ok(DecodedPositiveNumber { end_index: letter_i, number: nr });
        }
        print!("tail {value}<{follow_1n} "); //TODO @mverleg: TEMPORARY! REMOVE THIS!
        let addition = multiplier
            .checked_mul(value.saturating_add(block_addition))
            .ok_or(DecodeError::TooLarge)?;
        nr = nr
            .checked_add(addition)
            .ok_or(DecodeError::TooLarge)?;
        print!("nr:{nr} "); //TODO @mverleg: TEMPORARY! REMOVE THIS!
        multiplier = multiplier
            .checked_mul(follow_1n)
            .ok_or(DecodeError::TooLarge)?;

        non_close_letter_cnt_doubled += 1;
        block_i += 1;
        block_addition = 1;
    }
    print!("END-ERR:{}>={} ", block_i + (non_close_letter_cnt_doubled / 2), letters.len()); //TODO @mark: TEMPORARY! REMOVE THIS!
    Err(DecodeError::NoEndMarker)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecodedPositiveNumber {
    pub end_index: usize,
    pub number: u64,
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
            .filter(|letter| letter != &Text)
            .filter(&predicate)
            .collect();
        // Put Text and modifiers in the second half (if allowed) to keep the order more stable.
        for letter in Letter::modifiers()
            .into_iter()
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

    fn reverse_letter_values(letters: &[Letter]) -> Vec<u64> {
        let mut expected = vec![u64::MAX; 16];
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
    #[ignore] //TODO @mverleg: TEMPORARY! REMOVE THIS!
    fn openers() {
        assert_eq!(STRING_OPENERS, select_letters(|letter| letter.kind() != LetterKind::Modifier).as_slice());
    }

    /// After the start of the number, everything is allowed - encoutnering any of the second
    /// half of letters will signal the end of the number.
    #[test]
    #[ignore] //TODO @mverleg: TEMPORARY! REMOVE THIS!
    fn followers() {
        assert_eq!(STRING_FOLLOWERS, select_letters(|_letter| true).as_slice());
    }

    #[test]
    fn reverse_openers() {
        let expected = reverse_letter_values(&STRING_OPENERS);
        assert_eq!(STRING_OPENERS_VALUES, expected.as_slice());
    }

    #[test]
    fn reverse_followers() {
        let expected = reverse_letter_values(&STRING_FOLLOWERS);
        assert_eq!(STRING_FOLLOWER_VALUES, expected.as_slice());
    }
}

#[cfg(test)]
mod dynamic_width {
    use ::std::collections::HashSet;

    use super::*;
    use crate::compile::letter::LetterKind;
    use crate::compile::var_uint::DecodeError::TextNode;

    pub fn encode(nr: u64) -> Vec<Letter> {
        encode_uint_no_modifier_at_start(nr)
    }

    pub fn decode(letters: &[Letter]) -> DecodedPositiveNumber {
        decode_positive_int_static_width_avoid_modifiers(letters).unwrap()
    }

    fn encoding_to_str_for_debug(letters: &[Letter]) -> String {
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

    #[test]
    #[ignore] //TODO @mverleg: TEMPORARY! REMOVE THIS!
    fn print_all_encodings_for_debug() {
        for i in 0..=10_000 {
            let letters = encode(i);
            println!("{i}  {}", encoding_to_str_for_debug(&letters))
        }
    }

    #[test]
    #[ignore] //TODO @mverleg: TEMPORARY! REMOVE THIS!
    fn all_encodings_unique() {
        let n = 10_500_000;
        let mut seen = HashSet::with_capacity(n as usize);
        for i in 0..n {
            let enc = encode(i);
            println!(""); //TODO @mverleg: TEMPORARY! REMOVE THIS!
            assert!(enc.len() != 4 && enc.len() != 6, "nr {i} has impossible length {}", enc.len());
            assert!(seen.insert(enc), "nr {i} has same encoding as an earlier nr");
        }
    }

    #[test]
    fn positive_int_avoided_modifiers_encoding_examples() {
        assert_eq!(encode(0), vec![Asterisk]);
        assert_eq!(encode(4), vec![Colon]);
        assert_eq!(encode(5), vec![Number, Bracket]);
        assert_eq!(encode(9), vec![Plus, Bracket]);
        assert_eq!(encode(10), vec![Number, Colon]);
        assert_eq!(encode(40), vec![Number, Text]);
        assert_eq!(encode(44), vec![Plus, Text]);
        assert_eq!(encode(45), vec![Number, Number, Bracket]);
        assert_eq!(encode(85), vec![Number, Number, Colon]);
        assert_eq!(encode(364), vec![Plus, Right, Text]);
        assert_eq!(encode(365), vec![Number, Number, Number, Bracket]);
        assert_eq!(encode(1091), vec![Io, Io, Seq, Hat]);
        assert_eq!(encode(1878), vec![More, Slash, Asterisk, Question]);
        assert_eq!(encode(2462), vec![Seq, More, Plus, Tilde]);
        //TODO @mark: higher
    }

    #[test]
    fn positive_int_avoided_modifiers_decoding_examples() {
        assert_eq!(decode(&[Asterisk]), DecodedPositiveNumber { end_index: 0, number: 0 });
        assert_eq!(decode(&[Colon]), DecodedPositiveNumber { end_index: 0, number: 4 });
        assert_eq!(decode(&[Number, Bracket, Text]), DecodedPositiveNumber { end_index: 1, number: 5 });
        assert_eq!(decode(&[Plus, Bracket]), DecodedPositiveNumber { end_index: 1, number: 9 });
        assert_eq!(decode(&[Number, Colon, Right]), DecodedPositiveNumber { end_index: 1, number: 10 });
        assert_eq!(decode(&[Number, Text]), DecodedPositiveNumber { end_index: 1, number: 40 });
        assert_eq!(decode(&[Plus, Text]), DecodedPositiveNumber { end_index: 1, number: 44 });
        assert_eq!(decode(&[Number, Number, Bracket, Hat]), DecodedPositiveNumber { end_index: 2, number: 45 });
        assert_eq!(decode(&[Number, Number, Colon, Tilde, Io]), DecodedPositiveNumber { end_index: 2, number: 85 });
        assert_eq!(decode(&[Plus, Right, Text]), DecodedPositiveNumber { end_index: 2, number: 364 });
        assert_eq!(decode(&[Number, Number, Number, Bracket]), DecodedPositiveNumber { end_index: 3, number: 365 });
        assert_eq!(decode(&[Io, Io, Seq, Hat]), DecodedPositiveNumber { end_index: 3, number: 1091 });
        assert_eq!(decode(&[More, Slash, Asterisk, Question, Exclamation, Hash]), DecodedPositiveNumber { end_index: 3, number: 1878 });
        assert_eq!(decode(&[Seq, More, Plus, Tilde, Io]), DecodedPositiveNumber { end_index: 3, number: 2462 });
    }

    #[test]
    fn encode_and_decode_samples() {
        let nrs = (0..100).chain((0..10).map(|n| n * 2 - n));
        for nr in nrs {
            println!(""); //TODO @mverleg: TEMPORARY! REMOVE THIS!
            let enc = encode_uint_no_modifier_at_start(nr);
            println!("\n  {}", encoding_to_str_for_debug(&enc)); //TODO @mverleg: TEMPORARY! REMOVE THIS!
            let dec = decode_positive_int_static_width_avoid_modifiers(&enc).unwrap_or_else(|err| panic!("failed to decode {}, err {}", nr, err));
            println!(""); //TODO @mverleg: TEMPORARY! REMOVE THIS!
            assert_eq!(nr, dec.number);
        }
    }

    #[test]
    fn positive_int_avoid_modifiers_empty_input() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[]);
        assert_eq!(decode, Err(DecodeError::NoInput));
    }

    #[test]
    fn positive_int_starting_with_modifiers() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Letter::modifiers()[0], Asterisk]);
        assert_eq!(decode, Err(DecodeError::StarsWithModifier));
    }

    #[test]
    fn positive_int_non_terminated_number() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Io]);
        assert_eq!(decode, Err(DecodeError::NoEndMarker));
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Io, Io, Io]);
        assert_eq!(decode, Err(DecodeError::NoEndMarker));
    }

    #[test]
    fn positive_int_decode_with_text_opener() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Text, Hash]);
        assert_eq!(decode, Err(DecodeError::TextNode));
    }

    #[test]
    /// Because the number of letters is even, and Text is not allowed at the end, (at least) one other letter
    /// should also not be allowed. This checks that that is handled gracefully.
    fn positive_int_decode_with_unused_letter() {
        for list in [STRING_OPENERS.as_slice(), STRING_FOLLOWERS.as_slice()] {
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
                let dec = decode_positive_int_static_width_avoid_modifiers(&[Number, letter, Hash]);
                assert_eq!(dec, Err(DecodeError::UnexpectedNode), "list len: {}, letter: {}", list.len(), letter.symbol());
            }
        }
    }

    #[test]
    fn positive_int_overflow_in_decode() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Io; 100]);
        assert_eq!(decode, Err(DecodeError::TooLarge));
    }
}
