use ::std::env;
use ::std::fmt::Write;
use ::std::fs;
use ::std::path::PathBuf;

include!("src/compile/var_uint_build.rs");

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=dictionary.txt");
    let base_dict_str = fs::read_to_string("./dictionary.txt").unwrap();
    let base_dict_entries = base_dict_str.lines()
        .collect::<Vec<_>>();
    let code = generate_base_dict_code(&base_dict_entries);
    write_dict_code(&code);
}

fn generate_base_dict_code(base_dict_entries: &[&str]) -> String {
    let mut init_buffer = String::with_capacity(base_dict_entries.len() * 16);
    let mut cost_buffer = String::new();
    write!(init_buffer, "pub const DICT: [DictEntry; {}] = [\n", base_dict_entries.len()).unwrap();
    for (pos, entry) in base_dict_entries.into_iter().enumerate() {
        let cost = encode_snippet_len_estimate(pos.try_into().unwrap());
        let creator = match *entry {
            "$magic-backspace$" => {
                write!(cost_buffer, "\t\t\tDictEntry::Backspace => {cost},\n").unwrap();
                "DictEntry::Backspace".to_owned()
            },
            "$magic-backspace-front$" => {
                write!(cost_buffer, "\t\t\tDictEntry::BackspaceFront => {cost},\n").unwrap();
                "DictEntry::BackspaceFront".to_owned()
            },
            "$magic-newline$" => format!("s(\"\\n\",{cost})"),
            "$magic-capitalize-first$" => {
                write!(cost_buffer, "\t\t\tDictEntry::CapitalizeFirst => {cost},\n").unwrap();
                "DictEntry::CapitalizeFirst".to_owned()
            },
            "$magic-capitalize-all$" => {
                write!(cost_buffer, "\t\t\tDictEntry::CapitalizeAll => {cost},\n").unwrap();
                "DictEntry::CapitalizeAll".to_owned()
            },
            "$magic-reverse$" => {
                write!(cost_buffer, "\t\t\tDictEntry::Reverse => {cost},\n").unwrap();
                "DictEntry::Reverse".to_owned()
            },
            "$magic-unicode$" => {
                write!(cost_buffer, "\t\t\tDictEntry::UnicodeLookup => {cost},\n").unwrap();
                "DictEntry::UnicodeLookup".to_owned()
            },
            "\"" => format!("s(\"\\\"\",{cost})"),
            _ => if entry.ends_with("$magic-capitalize-next$") {
                format!("S(\"{}\",{cost})", entry.strip_suffix("$magic-capitalize-next$").unwrap())
            } else {
                assert!(!entry.contains("$magic"), "unknown: '{entry}'");
                format!("s(\"{entry}\",{cost})")
            },
        };
        write!(init_buffer, "\t{creator},\n").unwrap();
    }
    write!(init_buffer, "];\n\n").unwrap();
    write!(init_buffer, "impl DictEntry {{\n\tpub fn cost(&self) -> Cost {{\n\t\tmatch self {{\n\t\t\tDictEntry::Snippet {{ cost, .. }} => *cost,\n{cost_buffer}\t\t}}\n\t}}\n}}\n\n").unwrap();
    init_buffer
}

fn write_dict_code(code: &str) {
    let mut out_file = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_file.push("dict_init.rs");
    fs::write(out_file, code).expect("failed to write");
}
