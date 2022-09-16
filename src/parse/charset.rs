
#[derive(Debug, Clone)]
struct Char {
    byt: byte,
    chr: char,
    long: String,
}

impl Char {
    pub const fn new(short: byte, chr: char, long: impl Into<String>) -> Self {
        Char {
            byt,
            chr,
            long,
        }
    }
}

const A: Char = Char::new('a', "");
