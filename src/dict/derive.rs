use ::std::collections::hash_map::Entry;

use ::fnv::FnvBuildHasher;
use ::fnv::FnvHashMap;

use crate::common::TextTransformation;
use crate::dict::{CowDictStr, DictEntry, DictIx};
use crate::dict::Cost;
use crate::dict::entries::iter_snippets;

pub const MAX_TEXT_TRANSFORMS: usize = 2;
//TODO @mark: increase^

#[derive(Debug)]
pub struct DerivationInfo {
    pub derived_text: CowDictStr,
    pub original_index: DictIx,
    pub transformation: TextTransformation,
    pub cost: Cost,
}

#[derive(Debug)]
pub struct PartialDerivationInfo {
    pub original_index: DictIx,
    pub transformation: TextTransformation,
    pub cost: Cost,
}

pub fn with_derived_dict_entries(base_dict: &'static [DictEntry]) -> Vec<DerivationInfo> {
    let transformations = generate_transformations();
    debug_assert!(!transformations.is_empty());
    let capacity = base_dict.len() * transformations.len();
    let mut derivations: FnvHashMap<CowDictStr, PartialDerivationInfo> =
        FnvHashMap::with_capacity_and_hasher(capacity, FnvBuildHasher::default());
    for (original_index, snippet, snippet_cost) in iter_snippets(base_dict) {
        let original_cost = original_index;
        for transformation in transformations.iter().cloned() {
            let derived_text = transformation.apply_str(snippet);
            let original_index = original_index.try_into().expect("usize too small");
            let cost = snippet_cost + transformation.cost();
            debug_assert!(cost > 0);
            match derivations.entry(derived_text) {
                Entry::Occupied(mut existing) => {
                    if cost < existing.get().cost {
                        existing.insert(PartialDerivationInfo { original_index, transformation, cost });
                    }
                }
                Entry::Vacant(vacancy) => {
                    vacancy.insert(PartialDerivationInfo { original_index, transformation, cost });
                }
            }
        }
    }
    collect_transformations(derivations)
}

fn collect_transformations(derivations: FnvHashMap<CowDictStr, PartialDerivationInfo>) -> Vec<DerivationInfo> {
    derivations.into_iter()
        .map(|(dt, pdi)| DerivationInfo {
            derived_text: dt,
            original_index: pdi.original_index,
            transformation: pdi.transformation,
            cost: pdi.cost,
        })
        .collect()
}

#[cfg(test)]
pub fn all_transformations() -> Vec<TextTransformation> {
    generate_transformations()
}

fn generate_transformations() -> Vec<TextTransformation> {
    let mut transformations = vec![];
    for case_first in [false, true] {
        for case_all in [false, true] {
            for reverse in [false, true] {
                for pop_start in [0, 1, 2] {
                    for pop_end in [0, 1, 2, 3] {
                        let tt_count = case_first as u8 + case_all as u8 + reverse as u8 + pop_start + pop_end;
                        if tt_count as usize > MAX_TEXT_TRANSFORMS {
                            continue;
                        }
                        transformations.push(TextTransformation {
                            case_first,
                            case_all,
                            reverse,
                            pop_start,
                            pop_end,
                        })
                    }
                }
            }
        }
    }
    assert!(transformations.len() <= 100);
    transformations
}

#[derive(Debug)]
struct TransformationCost<'a> {
    index: usize,
    cost: u32,
    transformation: &'a TextTransformation,
}

#[cfg(test)]
mod tests {
    use ::std::cmp::max;

    use super::*;

    #[test]
    fn check_maximum_length() {
        let max = generate_transformations().iter()
            .map(|tt| tt.operation_indices().len())
            .max().unwrap();
        assert_eq!(max, MAX_TEXT_TRANSFORMS);
    }

    #[test]
    fn generate_transformations_operation_indices_length() {
        let mut capacity = 0;
        let mut longest = 0;
        for transformation in generate_transformations() {
            let indices = transformation.operation_indices();
            capacity = indices.capacity();
            longest = max(longest, indices.len());
        }
        assert_eq!(capacity, longest, "capacity of operation_indices() result is larger than needed");
    }
}
