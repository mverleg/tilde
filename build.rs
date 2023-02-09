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
    let mut buffer = format!("");
    write!(buffer, "pub const DICT: [DictEntry; {}] = [\n", base_dict_entries.len()).unwrap();
    for (pos, entry) in base_dict_entries.into_iter().enumerate() {
        let cost = encode_snippet_len_estimate(pos.try_into().unwrap());
        let creator = match *entry {
            "$magic-backspace$" => format!("DictEntry::Backspace({cost})"),
            "$magic-backspace-front$" => format!("DictEntry::BackspaceFront({cost})"),
            "$magic-newline$" => format!("s(\"\\n\",{cost})"),
            "$magic-capitalize-first$" => format!("DictEntry::CapitalizeFirst({cost})"),
            "$magic-capitalize-all$" => format!("DictEntry::CapitalizeAll({cost})"),
            "$magic-reverse$" => format!("DictEntry::Reverse({cost})"),
            "$magic-unicode$" => format!("DictEntry::UnicodeLookup({cost})"),
            "\"" => format!("s(\"\\\"\",{cost})"),
            _ => if entry.ends_with("$magic-capitalize-next$") {
                format!("S(\"{}\",{cost})", entry.strip_suffix("$magic-capitalize-next$").unwrap())
            } else {
                assert!(!entry.contains("$magic"), "unknown: '{entry}'");
                format!("s(\"{entry}\",{cost})")
            },
        };
        write!(buffer, "\t{creator},\n").unwrap();
    }
    write!(buffer, "];\n\n").unwrap();
    buffer
}

fn write_dict_code(code: &str) {
    let mut out_file = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_file.push("dict_init.rs");
    fs::write(out_file, code).expect("failed to write");
}
