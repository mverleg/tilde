use ::std::collections::HashSet;
use ::std::env;
use ::std::fs;
use ::std::path::PathBuf;

include!("src/common/dict_derive.rs");

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    derived_dict_entries();
}

fn derived_dict_entries() {
    println!("cargo:rerun-if-changed=src/common/dict_derive.rs");
    println!("cargo:rerun-if-changed=dictionary.txt");
    let mut out_file = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_file.push("dictionary_extended.txt");
    println!("cargo:rerun-if-changed={}", out_file.to_str().unwrap());
    let base_dict = fs::read_to_string("./dictionary.txt").unwrap();
    let original = base_dict.lines()
        .filter(|s| !s.is_empty() && s.chars().filter(|c| *c == '$').count() < 2)
        .collect::<Vec<_>>();
    let derived = collect_cap_derivations(&original);
    println!("would count: {} -> {}", original.len(), derived.len());
    let dict_str = sorted_join(derived);
    println!("{dict_str}");  //TODO @mark: TEMPORARY! REMOVE THIS!
    panic!();  //TODO @mark: TEMPORARY! REMOVE THIS!
    fs::write(out_file, dict_str).expect("failed to write");
}

fn collect_cap_derivations(original: &Vec<&str>) -> HashSet<DictDerivation> {
    original.iter()
        .flat_map(|line| cap_derivations(*line).into_iter())
        .collect::<HashSet<_>>()
}

fn sorted_join(derived: HashSet<DictDerivation>) -> String {
    let mut derived = derived.into_iter()
        .collect::<Vec<DictDerivation>>();
    derived.sort_by(|left, right| left.text.cmp(&right.text));
    let mut dict_str = String::with_capacity(derived.len() * 16);
    for deriv in derived {
        dict_str.push_str(&deriv.text);
        dict_str.push('\n')
    }
    dict_str
}
