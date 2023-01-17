use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::hash;
use ::std::hash::Hasher;

use crate::common::dict::{DictEntry, iter_snippets};
use crate::common::dict_str::CowDictStr;
use crate::common::TextTransformation;

#[derive(Debug)]
pub struct DerivationInfo {
    pub derived_text: CowDictStr,
    pub original_index: usize,
    pub transformation: TextTransformation,
    pub cost: u32,
}

impl hash::Hash for DerivationInfo {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(self.derived_text.as_ref().as_bytes())
    }
}

impl PartialEq for DerivationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.derived_text == other.derived_text
    }
}

impl Eq for DerivationInfo {}

pub fn with_derived_dict_entries(base_dict: &'static [DictEntry]) -> HashSet<DerivationInfo> {
    let transformations = generate_transformations();
    debug_assert!(!transformations.is_empty());
    let cap = base_dict.len() * transformations.len();
    let mut derivations = HashSet::with_capacity(cap);
    for (original_index, snippet) in iter_snippets(base_dict) {
        for transformation in &transformations {
            let derivation = DerivationInfo {
                derived_text: transformation.apply(snippet),
                original_index,
                transformation: transformation.clone(),
                cost: 0,  //TODO @mark:
            };
            let mut entry = derivations.get();
            entry.insert(derivation);
        }
    }
    derivations
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

// fn collect_cheapest_derivations(
//     base_dict_entries: &[&'static str],
//     transformations: &[TextTransformation],
// ) -> Vec<DerivationInfo> {
//     let mut min_costs: HashMap<CowDictStr, TransformationCost> = HashMap::new();
//     for (index, base_dict_entry) in base_dict_entries.iter().enumerate() {
//         if base_dict_entry.contains("$magic-") {
//             continue;
//         }
//         for transformation in transformations {
//             let new = TransformationCost {
//                 index,
//                 cost: 1,  //TODO @mark:
//                 transformation,
//             };
//             let deriv = transformation.apply(base_dict_entry);
//             match min_costs.entry(deriv) {
//                 Entry::Occupied(mut prev) => {
//                     if new.cost < prev.get().cost {
//                         prev.insert(new);
//                     }
//                 }
//                 Entry::Vacant(prev) => {
//                     prev.insert(new);
//                 }
//             }
//         }
//     }
//     let mut result = min_costs.into_iter()
//         .map(|(key, value)| DerivationInfo {
//             derived_text: key,
//             original_index: value.index,
//             cost: value.cost,
//             transformation: value.transformation.clone(),
//         })
//         .collect::<Vec<_>>();
//     result.sort_by(|deriv1, deriv2| deriv1.derived_text.cmp(&deriv2.derived_text));
//     result
// }

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
