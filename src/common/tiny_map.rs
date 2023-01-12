use ::std::collections::HashMap;

use ::tinyvec::ArrayVec;

const TINY_MAP_TRANSITION: usize = 2;

/// Map that allocates on the stack when it is small.
/// Only uses hashcode for 'big' maps, so behaviour might be different if Hash and Eq are inconsistent.
#[derive(Debug)]
pub enum TinyMap<K: Default, V: Default> {
    Small(ArrayVec<[(K, V); TINY_MAP_TRANSITION]>),
    Big(HashMap<K, V>),
}

impl <K: Default, V: Default> TinyMap<K, V> {
    pub fn new() -> Self {
        TinyMap::Small(ArrayVec::new())
    }
}
