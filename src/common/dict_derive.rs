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
) -> Vec<BuildDerivationInfo> {
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

fn generate_derived_dict_code(derivations: &[BuildDerivationInfo]) -> String {
    let mut buffer = String::new();
    let mut derivation_options = derivations.iter()
        .map(|deriv| (deriv.transformation.name(), &deriv.transformation))
        .collect::<HashMap<_, _>>()
        .into_iter()
        .collect::<Vec<_>>();
    derivation_options.sort_by(|entry1, entry2| entry1.0.cmp(&entry2.0));
    for (tt_name, tt) in derivation_options {
        write!(buffer, "#[inline]\nconst fn {}(derived_text: &'static str, original_index: usize, cost: usize) -> DerivationInfo {{
            let transformation = TextTransformation {{ case_first: {}, case_all: {}, reverse: {}, pop_start: {}, pop_end: {} }};
            DerivationInfo {{ derived_text, original_index, transformation, cost }} }}\n\n",
               tt_name, tt.case_first, tt.case_all, tt.reverse, tt.pop_start, tt.pop_end).unwrap();
    }

    write!(buffer, "pub const DERIVED: [DerivationInfo; {}] = [\n", derivations.len()).unwrap();
    for deriv in derivations.iter() {
        write!(buffer, "\t{}(", deriv.transformation.name()).unwrap();
        write!(buffer, "\"{}\", ", deriv.derived_text.replace("\"", "\\\"")).unwrap();
        write!(buffer, "{}, ", deriv.original_index).unwrap();
        write!(buffer, "{}", deriv.cost).unwrap();
        write!(buffer, "),\n").unwrap();
        //write!(buffer, "\t{creator},\n").unwrap();
    }
    write!(buffer, "];\n\n").unwrap();
    buffer
}