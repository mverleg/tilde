use ::std::str::from_utf8;

use ::base64::Engine;
use ::base64::engine::general_purpose::URL_SAFE_NO_PAD;

use crate::compile::Letter;
use crate::compile::LetterKind;
use crate::tilde_log;
use crate::TildeRes;

pub fn b64_encode(source: &[Letter]) -> TildeRes<String> {
    let mut bytes = Vec::with_capacity(source.len() * 4);
    let letters = source;
    let mut i = 0;
    while i + 1 < letters.len() {
        bytes.push(16 * letters[i].nr() + letters[i + 1].nr());
        i += 2;
    }
    if i < letters.len() {
        // This padding is based on the idea that fixed openers cannot appear at the end,
        // since there is nothing to open. But they could be assigned meaning in the future.
        //TODO @mverleg: They may actually already be part of a string, right? in which case we could add text close, but that requires parsing
        let pad = Letter::Io;
        debug_assert!(pad.kind() == LetterKind::FixedOpen);
        bytes.push(16 * letters[i].nr() + pad.nr())
    }
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

pub fn b64_decode(base64_source: &str) -> TildeRes<Vec<Letter>> {
    let Ok(src_bytes) = URL_SAFE_NO_PAD.decode(base64_source) else {
        return Err("base64 encoding not valid, alphabet should be A-Za-z0-9-_ without padding".to_string());
    };
    debug_assert!(src_bytes.len() % 2 == 0);
    let mut letters = Vec::with_capacity(src_bytes.len() / 2);
    for byte in src_bytes {
        letters.push(Letter::from_nr(byte / 16));
        letters.push(Letter::from_nr(byte % 16));
    }
    if letters.last().map(|pad| pad.kind() == LetterKind::FixedOpen).unwrap_or(false) {
        letters.pop();
    }
    Ok(letters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        use Letter::*;
        let inputs: &[&[Letter]] = &[
            // long, even
            &[Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon, Hat, Exclamation, Question, Hash, Tilde, Number, Text,],
            // long, odd
            &[Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon, Hat, Exclamation, Question, Hash, Tilde, Number, Text,],
            // duplicates
            &[Hat, Hat, Hat, Hat, Hat, ],
            // empty
            &[],
            // padding items to which padding is added
            &[Io, Io, Io,],
        ];
        for input in inputs {
            let enc = b64_encode(input).unwrap();
            let dec = b64_decode(&enc).unwrap();
            assert_eq!(*input, dec);
            assert!(enc.len() < dec.len());
        }
    }
}
