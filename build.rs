use ::std::fs;
use ::std::collections::HashSet;

include!("src/common/dict_derive.rs");

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    derived_dict_entries();
}

fn derived_dict_entries() {
    println!("cargo:rerun-if-changed=src/common/dict_derive.rs");
    println!("cargo:rerun-if-changed=dictionary.txt");
    let base_dict = fs::read_to_string("./dictionary.txt").unwrap();
    let original = base_dict.lines().collect::<Vec<_>>();
    let derived = collect_derivations(&original);
    println!("would count: {} -> {}", original.len(), derived.len());
    let dict_str = sorted_join(derived);
    fs::write("dictionary_extended.txt", dict_str).expect("failed to write");
}

fn collect_derivations(original: &Vec<&str>) -> HashSet<String> {
    let mut derived = original.iter()
        .filter(|s| !s.is_empty() && s.chars().filter(|c| *c == '$').count() < 2)
        .map(|s| (*s).to_owned())
        .collect::<HashSet<String>>();
    for line in original.iter() {
        for deriv in derivations((*line).to_owned()) {
            derived.insert(deriv.text.clone());
        }
    }
    derived
}

fn sorted_join(derived: HashSet<String>) -> String {
    let mut derived = derived.into_iter()
        .collect::<Vec<_>>();
    derived.sort();
    let mut dict_str = String::with_capacity(derived.len() * 16);
    for deriv in derived {
        dict_str.push_str(&deriv);
        dict_str.push('\n')
    }
    dict_str
}
