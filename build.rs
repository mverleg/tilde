use ::std::fs::read_to_string;

fn main() {

}

fn derived_dict_entries() {
    let base_dict = read_to_string("./dictionary.txt").unwrap();
    for line in base_dict.lines() {
        //derivations
    }
}
