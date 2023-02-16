use ::std::str::from_utf8;

use ::base64::Engine;
use ::base64::engine::general_purpose::URL_SAFE_NO_PAD;

use crate::{tilde_log, TildeRes};
use crate::compile::Letter;

pub fn b64_encode(source: Vec<Letter>) -> TildeRes<String> {
    let mut bytes = Vec::with_capacity(source.len() * 4);
    let letters = source;
    let mut i = 0;
    while i + 1 < letters.len() {
        bytes.push(16 * letters[i].nr() + letters[i + 1].nr());
        i += 2;
    }
    if i < letters.len() {
        let pad = Letter::Io;
        debug_assert!(pad.kind() == LetterKind::FixedOpen);
        bytes.push(16 * letters[i].nr() + pad)
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

