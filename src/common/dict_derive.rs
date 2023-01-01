use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use crate::common::text_trans::DictStr;
use crate::common::TextTransformation;

#[derive(Debug)]
pub struct DerivationInfo {
    pub derived_text: DictStr,
    pub original_index: usize,
    pub transformation: TextTransformation,
    pub cost: usize,
}

fn collect_cheapest_derivations<'a>(
    base_dict_entries: &[&'a str],
    transformations: &'a [TextTransformation]
) -> Vec<DerivationInfo> {
    let mut min_costs: HashMap<String, (usize, Cost, &TextTransformation)> = HashMap::new();
    for (index, entry) in base_dict_entries.iter().enumerate() {
        if entry.contains("$magic-") {
            continue
        }
        for trans in transformations {
            let deriv = trans.apply(entry);
            let cost: Cost = 1;  //TODO @mark:
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
        .map(|(key, value)| BuildDerivationInfo {
            original_index: value.0,
            derived_text: key,
            cost: value.1,
            transformation: value.2.clone(),
        })
        .collect::<Vec<_>>();
    result.sort_by(|deriv1, deriv2| deriv1.derived_text.cmp(&deriv2.derived_text));
    result
}
