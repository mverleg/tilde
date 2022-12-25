use ::std::borrow::Cow;
use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use ::std::env;
use ::std::fmt::Write;
use ::std::fs;
use ::std::path::PathBuf;

use crate::text_trans::DerivationInfo;
use crate::text_trans::TextTransformation;

// use ::std::path::PathBuf;

type Cost = usize;

mod text_trans {
    include!("src/common/text_trans.rs");
}
mod build_text {
    include!("src/common/build_text.rs");
}

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=dictionary.txt");
    let base_dict_str = fs::read_to_string("./dictionary.txt").unwrap();
    let base_dict_entries = base_dict_str.lines()
        .collect::<Vec<_>>();
    let mut code = generate_base_dict_code(&base_dict_entries);
    let derivation_options = generate_derivation_options();
    let derivations = collect_cheapest_derivations(&base_dict_entries, &derivation_options);
    code.push_str(&generate_derived_dict_code(&derivations));
    write_dict_code(&code);
}

fn generate_base_dict_code(base_dict_entries: &[&str]) -> String {
    let mut buffer = format!("");
    write!(buffer, "pub const DICT: [DictEntry; {}] = [\n", base_dict_entries.len()).unwrap();
    for entry in base_dict_entries.iter() {
        let creator = match *entry {
            "$magic-backspace$" => "DictEntry::Backspace".to_owned(),
            "$magic-newline$" => "s(\"\\n\")".to_owned(),
            "$magic-capitalize-first$" => "DictEntry::CapitalizeFirst".to_owned(),
            "$magic-capitalize all$" => "DictEntry::CapitalizeAll".to_owned(),
            "\"" => "s(\"\\\"\")".to_owned(),
            _ => if entry.ends_with("$capitalize-next$") {
                format!("S(\"{}\")", entry.strip_suffix("$capitalize-next$").unwrap())
            } else {
                assert!(!entry.contains("$magic"));
                format!("s(\"{}\")", entry)
            },
        };
        write!(buffer, "\t{creator},\n").unwrap();
    }
    write!(buffer, "];\n\n").unwrap();
    buffer
}

fn generate_derived_dict_code(derivations: &[DerivationInfo]) -> String {
    let mut buffer = String::new();
    let mut derivation_options = derivations.iter()
        .map(|deriv| (deriv.transformation.name(), &deriv.transformation))
        .collect::<HashMap<_, _>>()
        .into_iter()
        .collect::<Vec<_>>();
    derivation_options.sort_by(|entry1, entry2| entry1.0.cmp(&entry2.0));
    for (tt_name, tt) in derivation_options {
        write!(buffer, "#[inline]\nfn {}() -> TextTransformation {{ \
        TextTransformation {{ case_first: {}, case_all: {}, reverse: {}, pop_start: {}, pop_end: {} }} }}\n\n",
               tt_name, tt.case_first, tt.case_all, tt.reverse, tt.pop_start, tt.pop_end).unwrap();
    }

    write!(buffer, "pub const DICT: [DerivationInfo; {}] = [\n", derivations.len()).unwrap();
    for deriv in derivations.iter() {
        todo!();
        //write!(buffer, "\t{creator},\n").unwrap();
    }
    write!(buffer, "];\n\n").unwrap();

    // buffer.push_str(&format!("pub const DERIVED_DICT: [DictEntry; {}] = [\n", base_dict_entries.len()));
    // for entry in base_dict_entries.iter() {
    //     let creator = match *entry {
    //         "$magic-backspace$" => "DictEntry::Backspace".to_owned(),
    //         "$magic-newline$" => "s(\"\\n\")".to_owned(),
    //         "$magic-capitalize-first$" => "DictEntry::CapitalizeFirst".to_owned(),
    //         "$magic-capitalize all$" => "DictEntry::CapitalizeAll".to_owned(),
    //         "\"" => "s(\"\\\"\")".to_owned(),
    //         _ => if entry.ends_with("$capitalize-next$") {
    //             format!("S(\"{}\")", entry.strip_suffix("$capitalize-next$").unwrap())
    //         } else {
    //             assert!(!entry.contains("$magic"));
    //             format!("s(\"{}\")", entry)
    //         },
    //     };
    //     buffer.push_str(&format!("\t{creator},\n"))
    // }
    // buffer.push_str("];\n\n");
    buffer
}

fn generate_derivation_options() -> Vec<TextTransformation> {
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

fn collect_cheapest_derivations<'a>(
    base_dict_entries: &[&'a str],
    transformations: &'a [TextTransformation]
) -> Vec<DerivationInfo> {
    let mut min_costs: HashMap<Cow<str>, (usize, Cost, &TextTransformation)> = HashMap::new();
    for (index, entry) in base_dict_entries.iter().enumerate() {
        if entry.starts_with("$magic-") {
            continue
        }
        for trans in transformations {
            let deriv = trans.apply(entry);
            let cost: Cost = 1;  //TODO @mark:
            match min_costs.entry(deriv) {
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
            original_index: value.0,
            derived_text: key.into_owned(),
            cost: value.1,
            transformation: value.2.clone(),
        })
        .collect::<Vec<_>>();
    result.sort_by(|deriv1, deriv2| deriv1.derived_text.cmp(&deriv2.derived_text));
    result
}

fn write_dict_code(code: &str) {
    let mut out_file = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_file.push("dict_init.rs");
    println!("cargo:rerun-if-changed={}", out_file.to_str().unwrap());
    fs::write(out_file, code).expect("failed to write");
}

//
// pub const MAX_BACKSPACE: u8 = 3;
//
// //TODO @mark: reverse
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum CapitalizeKind {
//     None,
//     First,
//     All,
// }
//
// impl CapitalizeKind {
//     pub fn apply(&self, input: &str) -> String {
//         match self {
//             CapitalizeKind::None => input.to_owned(),
//             CapitalizeKind::First => {
//                 let mut iter = input.chars();
//                 let mut text = match iter.next() {
//                     Some(c) => toggle_case(c),
//                     None => return input.to_owned(),
//                 };
//                 iter.for_each(|c| text.push(c));
//                 text
//             }
//             CapitalizeKind::All => input.chars()
//                 .map(toggle_case)
//                 .collect(),
//         }
//     }
// }
//
// fn toggle_case(input: char) -> String {
//     //TODO @mark: so many allocations... (because upper case may be several chars long)
//     let up = input.to_uppercase().collect();
//     if input.to_string() != up {
//         return up;
//     }
//     input.to_lowercase().collect()
// }
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct CapDerivationSteps {
//     capitalize_self: CapitalizeKind,
// }
//
// impl CapDerivationSteps {
//     pub fn from_repr(repr: char) -> Self {
//         match repr {
//             'a' => CapDerivationSteps { capitalize_self : CapitalizeKind::None },
//             'b' => CapDerivationSteps { capitalize_self : CapitalizeKind::First },
//             'c' => CapDerivationSteps { capitalize_self : CapitalizeKind::All },
//             _ => unimplemented!(),
//         }
//     }
//
//     pub fn cost(&self) -> usize {
//         1  //TODO @mark: TEMPORARY! REMOVE THIS!
//     }
//
//     pub fn to_repr(&self) -> char {
//         match self.capitalize_self {
//             CapitalizeKind::None => 'a',
//             CapitalizeKind::First => 'b',
//             CapitalizeKind::All => 'c',
//         }
//     }
// }
//
// pub fn cap_derivations(base_text: &str) -> Vec<DictDerivation> {
//     //TODO @mark: base_Text borrow? can DictDerivation still be without lifetime?
//     let mut deriv = vec![];
//     eprintln!("todo remove CapitalizeKind::None here");
//     for cap in [CapitalizeKind::None, CapitalizeKind::First, CapitalizeKind::All] {
//         let cap_text = cap.apply(base_text).clone();
//         deriv.push(DictDerivation {
//             text: cap_text,
//             steps: CapDerivationSteps {
//                 capitalize_self: cap,
//             }
//         });
//     }
//     deriv
// }
//
// #[cfg(test)]
// mod capitalize {
//     use super::*;
//
//     #[test]
//     fn empty() {
//         assert_eq!(CapitalizeKind::None.apply(""), "");
//         assert_eq!(CapitalizeKind::First.apply(""), "");
//         assert_eq!(CapitalizeKind::All.apply(""), "");
//     }
//
//     #[test]
//     fn none() {
//         assert_eq!(CapitalizeKind::None.apply("a"), "a");
//         assert_eq!(CapitalizeKind::None.apply("abc"), "abc");
//         assert_eq!(CapitalizeKind::None.apply("ABC"), "ABC");
//         assert_eq!(CapitalizeKind::None.apply("🦀"), "🦀");
//     }
//
//     #[test]
//     fn first() {
//         assert_eq!(CapitalizeKind::First.apply("a"), "A");
//         assert_eq!(CapitalizeKind::First.apply("abc"), "Abc");
//         assert_eq!(CapitalizeKind::First.apply("ABC"), "aBC");
//         assert_eq!(CapitalizeKind::First.apply("🦀"), "🦀");
//     }
//
//     #[test]
//     fn all() {
//         assert_eq!(CapitalizeKind::All.apply("A"), "a");
//         assert_eq!(CapitalizeKind::All.apply("abc"), "ABC");
//         assert_eq!(CapitalizeKind::All.apply("ABC"), "abc");
//         assert_eq!(CapitalizeKind::All.apply("🦀"), "🦀");
//     }
// }
//
// #[cfg(test)]
// mod encoding {
//     use ::strum::IntoEnumIterator;
//
//     use crate::common::dict::DictEntry;
//
//     use super::*;
//
//     #[test]
//     fn all_specials_encountered() {
//         for expect in CapDerivationSteps::iter() {
//             if matches!(expect, DictEntry::Snippet { .. }) {
//                 continue
//             }
//             assert!(seen.contains(&expect), "expected in dict: {expect:?}");
//         }
//     }
// }
//
// fn derived_dict_entries() {
//     println!("cargo:rerun-if-changed=src/common/dict_derive.rs");
//     println!("cargo:rerun-if-changed=dictionary.txt");
//     let mut out_file = PathBuf::from(env::var("OUT_DIR").unwrap());
//     out_file.push("dictionary_extended.txt");
//     println!("cargo:rerun-if-changed={}", out_file.to_str().unwrap());
//     let base_dict = fs::read_to_string("./dictionary.txt").unwrap();
//     let original = base_dict.lines()
//         .filter(|s| !s.is_empty() && s.chars().filter(|c| *c == '$').count() < 2)
//         .collect::<Vec<_>>();
//     let derived = collect_cap_derivations(&original);
//     println!("would count: {} -> {}", original.len(), derived.len());
//     let dict_str = sorted_join(derived);
//     fs::write(out_file, dict_str).expect("failed to write");
// }
//
// fn collect_cap_derivations(original: &Vec<&str>) -> HashSet<DictDerivation> {
//     original.iter()
//         .flat_map(|line| cap_derivations(*line).into_iter())
//         .collect::<HashSet<_>>()
// }
//
// fn sorted_join(derived: HashSet<DictDerivation>) -> String {
//     let mut derived = derived.into_iter()
//         .collect::<Vec<DictDerivation>>();
//     derived.sort_by(|left, right| left.text.cmp(&right.text));
//     let mut dict_str = String::with_capacity(derived.len() * 16);
//     for deriv in derived {
//         dict_str.push_str(&deriv.text);
//         dict_str.push('\n')
//     }
//     dict_str
// }
