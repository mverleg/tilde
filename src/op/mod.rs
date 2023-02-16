pub use ::std::slice;
use ::std::vec;
use ::std::hash;
use ::std::hash::Hasher;

mod ops;

pub const fn op_options() -> &'static [Op] {
    todo!()
}

#[derive(Debug, Clone)]
pub struct Op {
    id: u16,
}

impl PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Op {}

impl hash::Hash for Op {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u16(self.id)
    }
}

//TODO @mark tests:
// incrementing ids
// unique long and short names
// correct tokens in long and short names
// parsing and rendering are eachothers opposite
// lookup by id? for dense encoding?

