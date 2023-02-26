use ::std::fmt;
use ::std::hash;
use ::std::hash::Hasher;

use ::tinyvec::TinyVec;
use ::tinyvec::TinyVecIterator;

use crate::compile::Letter;
use crate::compile::LetterKind;
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

    pub fn new1(letter: Letter) -> Self {
        let mut content = GolfWordContent::new();
        content.push(letter);
        Self::new(content)
    }

    pub fn new2(letter1: Letter, letter2: Letter) -> Self {
        let mut content = GolfWordContent::new();
        content.push(letter1);
        content.push(letter2);
        Self::new(content)
    }

    pub fn new3(letter1: Letter, letter2: Letter, letter3: Letter) -> Self {
        let mut content = GolfWordContent::new();
        content.push(letter1);
        content.push(letter2);
        content.push(letter3);
        Self::new(content)
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
    let mut scale = 1;
    let mut id: u32 = 0;
    for letter in letters {
        let letter_val: WordId = letter.nr().into();
        id = id.overflowing_add(letter_val.saturating_mul(scale)).0;
        let new_scale_res = scale.overflowing_mul(Letter::option_count() as WordId);
        scale = new_scale_res.0;
        if new_scale_res.1 {
            scale += 1;
            //TODO @mark: useful? does this help entropy?
        }
    }
    id
}

#[cfg(test)]
mod id_tests {
    use super::*;

    #[test]
    fn non_literal() {
        let id = calculate_id(&[Letter::Seq, Letter::Number, Letter::Hat]);
        assert_eq!(id, 2529);
    }

    #[test]
    fn literal() {
        let id = calculate_id(&[Letter::Text, Letter::Seq, Letter::Seq, Letter::Seq, Letter::Seq, Letter::Seq,
            Letter::Seq, Letter::Seq, Letter::Seq, Letter::Seq, Letter::Seq, Letter::Seq, Letter::Seq, Letter::Seq]);
        assert_eq!(id, 287449648);
    }
}


//TODO @mark: test that no words are longer than MAX_WORD_LENGTH
//TODO @mark: test that at least one word is as long as MAX_WORD_LENGTH
