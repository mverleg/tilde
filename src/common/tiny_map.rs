use ::std::collections::HashMap;
use std::hash::Hash;

use ::tinyvec::ArrayVec;

const TINY_MAP_TRANSITION: usize = 2;

/// Grow-only map that allocates on the stack when it is small.
/// Only uses hashcode for 'big' maps, so behaviour might be different if Hash and Eq are inconsistent.
/// For now, the `get` method takes an owned key instead of ref, because in current use it is small.
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

impl <K: Default + Eq + Hash, V: Default> TinyMap<K, V> {
    pub fn get(&self, key: K) -> Option<&V> {
        match self {
            TinyMap::Small(vec) => {
                for elem in vec {
                    if key == elem.0 {
                        return Some(&elem.1)
                    }
                }
                None
            }
            TinyMap::Big(map) => map.get(&key),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let should_upgrade = match self {
            TinyMap::Small(vec) => {
                if let Some(_) = vec.try_push((key, value)) {
                    // need to switch from Small to Big
                    true
                } else {
                    false
                }
            }
            TinyMap::Big(map) => {
                map.insert(key, value);
                false
            },
        };
        if should_upgrade {
            let mut map = HashMap::new();
            for (k, v) in vec {
                map.insert(k, v);
            }
            map.insert(key, value);
            *self = TinyMap::Big(map)
        }
    }
}
