use ::std::hash;
use ::std::hash::Hasher;
use ::std::mem::size_of;
use ::tinyvec::ArrayVec;
use crate::compile::Letter;

const MAX_WORD_LENGTH: usize = 3;
type GolfWordContent = ArrayVec<[Letter; MAX_WORD_LENGTH]>;
type WordId = u32;

#[derive(Debug, Clone)]
pub struct GolfWord {
    letters: GolfWordContent,
    /// unique id that doesn't have a meaning but can be used for fast, stable
    /// equality, hash or possibly serialization (assuming the size never needs to grow).
    id: WordId,
}

impl GolfWord {
    pub fn new(letters: GolfWordContent) -> Self {
        debug_assert!(!letters.is_empty());
        //TODO @mark: validate that this is a real word
        let mut hash = 0;
        GolfWord {
            letters,
            id: hash,
        }
    }
}

impl From<GolfWordContent> for GolfWord {
    fn from(value: GolfWordContent) -> Self {
        GolfWord::new(value)
    }
}

impl AsRef<[Letter]> for GolfWord {
    fn as_ref(&self) -> &[Letter] {
        &self.letters
    }
}

impl PartialEq for GolfWord {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for GolfWord {}

impl hash::Hash for GolfWord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.id)
    }
}

pub fn calculate_id(letters: &[Letter]) -> WordId {
    debug_assert!(letters.len() <= 2 * size_of::<WordId>());
    let mut scale = 1;
    let mut id = 0;
    for letter in letters {
        let letter_val: WordId = letter.nr().into();
        id += letter_val * scale;
        scale *= Letter::option_count() as WordId;
    }
    id
}

//TODO @mark: test that no words are longer than MAX_WORD_LENGTH
//TODO @mark: test that at least one word is as long as MAX_WORD_LENGTH
