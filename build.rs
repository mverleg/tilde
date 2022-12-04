use ::std::fs::read_to_string;
use ::std::collections::HashSet;

include!("src/common/dict_derive.rs");

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    derived_dict_entries();
}

fn derived_dict_entries() {
    println!("cargo:rerun-if-changed=src/common/dict_derive.rs");
    println!("cargo:rerun-if-changed=dictionary.txt");
    let base_dict = read_to_string("./dictionary.txt").unwrap();
    let lines = base_dict.lines().collect::<Vec<_>>();
    let mut known = lines.iter()
        .map(|s| (*s).to_owned())
        .collect::<HashSet<String>>();
    for line in lines.iter() {
        for deriv in derivations((*line).to_owned()) {
            if known.contains(&deriv.text) {
                //TODO @mark:
            } else {
                known.insert(deriv.text.clone());
            }
        }
    }
    println!("would count: {} -> {}", lines.len(), known.len());
}
