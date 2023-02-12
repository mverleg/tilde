use ::std::borrow::BorrowMut;
use ::std::collections::HashMap;
use ::std::hash::Hash;

use ::tinyvec::ArrayVec;

const TINY_MAP_TRANSITION: usize = 3;

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
                for (k, v) in vec {
                    if key == *k {
                        return Some(v)
                    }
                }
                None
            }
            TinyMap::Big(map) => map.get(&key),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        match self {
            TinyMap::Small(ref mut vec) => {
                // overwrite existing value?
                for elem in &mut *vec {
                    if key == elem.0 {
                        *elem = (key, value);
                        return;
                    }
                }
                // need to switch from Small to Big?
                if let Some((key, value)) = vec.try_push((key, value)) {
                    let mut map = HashMap::new();
                    for (k, v) in vec.drain(..) {
                        map.insert(k, v);
                    }
                    map.insert(key, value);
                    *self = TinyMap::Big(map)
                }
            }
            TinyMap::Big(map) => {
                map.insert(key, value);
            },
        };
    }

    pub fn len(&self) -> usize {
        match self {
            TinyMap::Small(vec) => vec.len(),
            TinyMap::Big(map) => map.len(),
        }
    }
}

#[cfg(test)]
mod tiny_map_test {
    use super::*;

    #[test]
    fn unique() {
        let mut map = TinyMap::new();
        for i in 0..=20 {
            map.insert(i, i * 10);
            assert_eq!(map.get(i), Some(&(10 * i)));
            assert_eq!(map.get(i + 1), None);
            assert_eq!(map.len(), i + 1);
        }
    }

    #[test]
    fn duplicate() {
        let mut map = TinyMap::new();
        assert_eq!(map.len(), 0);
        for i in 0..=20 {
            map.insert("hello world", i);
            assert_eq!(map.get("hello world"), Some(&i));
            assert_eq!(map.len(), 1);
        }
    }
}
