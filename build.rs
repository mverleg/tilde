use ::std::env;
use ::std::fmt::Write;
use ::std::fs;
use ::std::path::PathBuf;

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
    for entry in base_dict_entries.iter() {
        let creator = match *entry {
            "$magic-backspace$" => "DictEntry::Backspace".to_owned(),
            "$magic-newline$" => "s(\"\\n\")".to_owned(),
            "$magic-capitalize-first$" => "DictEntry::CapitalizeFirst".to_owned(),
            "$magic-capitalize all$" => "DictEntry::CapitalizeAll".to_owned(),
            "\"" => "s(\"\\\"\")".to_owned(),
            _ => if entry.ends_with("$magic-capitalize-next$") {
                format!("S(\"{}\")", entry.strip_suffix("$magic-capitalize-next$").unwrap())
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

fn write_dict_code(code: &str) {
    let mut out_file = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_file.push("dict_init.rs");
    println!("cargo:rerun-if-changed={}", out_file.to_str().unwrap());
    fs::write(out_file, code).expect("failed to write");
}
