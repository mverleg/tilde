use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use crate::common::dict::DictEntry;
use crate::common::text_trans::DictStr;
use crate::common::TextTransformation;

#[derive(Debug)]
pub struct DerivationInfo {
    pub derived_text: DictStr,
    pub original_index: usize,
    pub transformation: TextTransformation,
    pub cost: u32,
}

pub fn with_derived_dict_entries(base_dict: &[DictEntry]) -> Vec<DerivationInfo> {
    let transformations = generate_transformations();
    todo!();  //TODO @mark:
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

fn collect_cheapest_derivations(
    base_dict_entries: &[&str],
    transformations: &[TextTransformation]
) -> Vec<DerivationInfo> {
    let mut min_costs: HashMap<String, TransformationCost> = HashMap::new();
    for (index, entry) in base_dict_entries.iter().enumerate() {
        if entry.contains("$magic-") {
            continue
        }
        for trans in transformations {
            let deriv = trans.apply(entry);
            let cost: u32 = 1;  //TODO @mark:
            match min_costs.entry(deriv.into_owned()) {
                Entry::Occupied(mut prev) => {
                    if cost < prev.get().0 {
                        prev.insert((index, cost, trans));
                    }
                }
                Entry::Vacant(prev) => {
                    prev.insert((index, cost, trans));
                },
            }
        }
    }
    let mut result = min_costs.into_iter()
        .map(|(key, value)| DerivationInfo {
            derived_text: key,
            original_index: value.index,
            cost: value.cost,
            transformation: value.transformation.clone(),
        })
        .collect::<Vec<_>>();
    result.sort_by(|deriv1, deriv2| deriv1.derived_text.cmp(&deriv2.derived_text));
    result
}