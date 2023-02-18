use ::std::fmt;
use ::std::hash;
use ::std::hash::Hasher;
use ::std::mem::size_of;

use ::tinyvec::TinyVec;
use ::tinyvec::TinyVecIterator;

use crate::compile::{Letter, LetterKind};
use crate::TildeRes;

// max length does not apply to literals, which are unbounded,
// which is why TinyVec instead of ArrayVec is used
const MAX_WORD_LENGTH: usize = 3;
pub type GolfWordContent = TinyVec<[Letter; MAX_WORD_LENGTH]>;
type WordId = u32;

#[derive(Debug, Clone)]
pub struct GolfWord {
    letters: GolfWordContent,
    /// unique id that doesn't have a meaning but can be used for fast, stable
    /// equality, hash or possibly serialization (assuming the size never needs to grow).
    id: WordId,
}

impl GolfWord {
    pub fn try_new(letters: GolfWordContent) -> TildeRes<Self> {
        use LetterKind::*;
        debug_assert!(!letters.is_empty());
        match letters[0].kind() {
            Literal => {}
            VariableOpen => {
                for following in letters.iter().skip(1) {
                    if following.kind() != Modifier {
                        return Err(format!("golf identifier starting with variable opener ({}) must be followed by only modifiers (found '{}')", letters[0], following))
                    }
                }
            }
            FixedOpen => {
                for following in letters.iter().skip(2) {
                    if following.kind() != Modifier {
                        return Err(format!("golf identifier starting with variable opener ({}) must be followed by only modifiers (found '{}')", letters[0], following))
                    }
                }
            }
            Modifier => return Err(format!("golf identifier cannot start with modifier token {}", letters[0]))
        }
        let hash = calculate_id(&letters);
        Ok(GolfWord {
            letters,
            id: hash,
        })
    }

    pub fn new(letters: GolfWordContent) -> Self {
        match Self::try_new(letters) {
            Ok(gw) => gw,
            Err(err) => panic!("{err}"),
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

impl IntoIterator for GolfWord {
    type Item = Letter;
    type IntoIter = TinyVecIterator<[Letter; 3]>;

    fn into_iter(self) -> Self::IntoIter {
        self.letters.into_iter()
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

impl fmt::Display for GolfWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for letter in &self.letters {
            write!(f, "{}", letter)?
        }
        Ok(())
    }
}

pub fn calculate_id(letters: &[Letter]) -> WordId {
    debug_assert!(letters.len() <= 2 * size_of::<WordId>());
    //TODO @mark: this condition isn't true for literals, refactor?
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
