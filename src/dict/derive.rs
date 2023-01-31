use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::hash;
use ::std::hash::Hasher;

use ::fnv::{FnvBuildHasher, FnvHasher, FnvHashMap};

use crate::common::TextTransformation;
use crate::dict::{CowDictStr, DictEntry, INDX};
use crate::dict::compress::COST;
use crate::dict::entries::iter_snippets;

#[derive(Debug)]
pub struct DerivationInfo {
    pub derived_text: CowDictStr,
    pub original_index: INDX,
    pub transformation: TextTransformation,
    pub cost: COST,
}

#[derive(Debug)]
pub struct PartialDerivationInfo {
    pub original_index: INDX,
    pub transformation: TextTransformation,
    pub cost: COST,
}

pub fn with_derived_dict_entries(base_dict: &'static [DictEntry]) -> Vec<DerivationInfo> {
    let transformations = generate_transformations();
    debug_assert!(!transformations.is_empty());
    let capacity = base_dict.len() * transformations.len();
    let mut derivations: FnvHashMap<CowDictStr, PartialDerivationInfo> =
        FnvHashMap::with_capacity_and_hasher(capacity, FnvBuildHasher::default());
    for (original_index, snippet) in iter_snippets(base_dict) {
        for transformation in &transformations {
            let derived_text = transformation.apply_str(snippet);
            insert_if_cheapest(&mut derivations, derived_text, |cost| PartialDerivationInfo {
                original_index: original_index.try_into().expect("usize too small"),
                transformation: transformation.clone(),
                cost,
            })
        }
    }
    collect_transformations(derivations)
}

fn insert_if_cheapest(
        derivations: &mut FnvHashMap<CowDictStr, PartialDerivationInfo>,
        derived_text: CowDictStr,
        creator: impl Fn(COST) -> PartialDerivationInfo) {
    let new_cost = (100 / derived_text.as_ref().len()) as COST;  //TODO @mverleg: TEMPORARY! REMOVE THIS!
    match derivations.entry(derived_text) {
        Entry::Occupied(mut existing) => {
            if new_cost < existing.get().cost {
                existing.insert(creator(new_cost));
            }
        }
        Entry::Vacant(vacancy) => {
            vacancy.insert(creator(new_cost));
        }
    }
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

fn generate_transformations() -> Vec<TextTransformation> {
    let mut transformations = vec![];
    for case_first in [false, true] {
        for case_all in [false] {
            //TODO @mark: ^
            for reverse in [false] {
                //TODO @mark: ^
                for pop_start in [0] {
                    //TODO @mark: ^
                    for pop_end in [0, 1, 2, 3] {
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
