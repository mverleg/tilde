use ::std::fs::read_to_string;
use ::std::collections::HashSet;

include!("src/common/dict_derive.rs");

fn main() {
    derived_dict_entries();
}

fn derived_dict_entries() {
    let base_dict = read_to_string("./dictionary.txt").unwrap();
    let mut known = base_dict.lines()
        .map(|s| s.to_owned())
        .collect::<HashSet<_>>();
    for line in base_dict.lines() {
        for deriv in derivations(line.to_owned()) {
            if known.contains(&deriv.text) {
                todo!();
            } else {
                known.insert(deriv.text.clone());
            }
        }
    }
}
