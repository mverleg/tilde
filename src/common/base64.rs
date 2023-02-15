use ::std::str::from_utf8;
use ::base64::engine::general_purpose::URL_SAFE_NO_PAD;

use crate::{tilde_log, TildeRes};
use crate::compile::Letter;

pub fn b64_encode(source: Vec<Letter>) -> TildeRes<Vec<Letter>> {
    let mut bytes = Vec::with_capacity(source.len() * 4);
    let letters = source;
    let mut i = 0;
    while i + 1 < letters.len() {
        bytes.push(16 * letters[i].nr() + letters[i + 1].nr());
        i += 2;
    }
    if i < letters.len() {
        bytes.push(16 * letters[i].nr())
        //TODO @mark: need to to something to make the last letter not interpreted (or no-op)
    }
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

pub fn b64_decode(base64_source: &str) -> TildeRes<Vec<Letter>> {
    let Some(src_bytes) = URL_SAFE_NO_PAD.decode(base64_source) else {
        return Err("base64 encoding not valid, alphabet should be A-Za-z0-9-_ without padding".to_string());
    };
    let Ok(src) = from_utf8(&src_bytes) else {
        tilde_log!("base64 decoded bytes: {:?}", &src_bytes);
        return Err("source is not valid utf8 after base64-decoding; should contain valid, golfed tilde input, which is ascii".to_string())
    };
}