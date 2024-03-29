use ::std::env;
use ::std::fmt;
use ::std::fmt::Write;
use ::std::fs;
use ::std::path::PathBuf;
use ::std::process::Command;

include!("src/compile/var_uint_build.rs");

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=dictionary.txt");
    bin_example_tests().unwrap();
    let base_dict_str = fs::read_to_string("./dictionary.txt").unwrap();
    let base_dict_entries = base_dict_str.lines()
        .collect::<Vec<_>>();
    let code = generate_base_dict_code(&base_dict_entries);
    write_dict_code(&code);
}

fn find_all_bins() -> Vec<String> {
    // trigger error message to list all binaries
    let out = Command::new("cargo")
        .args(["run", "--bin"])
        .output()
        .expect("failed to execute process");
    String::from_utf8(out.stderr)
        .expect("cargo output not utf8")
        .lines()
        .skip(2)
        .map(|line| {
            line.trim().to_owned()
        })
        .filter(|bin| !bin.is_empty())
        .filter(|bin| !bin.contains("debug"))
        .collect()
}

fn find_all_examples() -> Vec<String> {
    let mut examples = Vec::new();
    if let Ok(examples_dir) = fs::read_dir("./examples") {
        for example_res in examples_dir {
            let path = example_res.unwrap()
                .file_name().to_str().unwrap().to_owned();
            if path.ends_with(".rs") {
                examples.push(path.strip_suffix(".rs").unwrap().to_owned());
            }
        }
    }
    examples
}

fn bin_example_tests() -> Result<(), fmt::Error> {
    println!("cargo:rerun-if-changed=examples");
    let bins = find_all_bins();
    let examples = find_all_examples();
    println!("bins: {}, examples: {}", bins.len(), examples.len());
    let mut code = String::new();
    writeln!(code, "\n// auto-generated by build.rs")?;
    writeln!(code, "#[cfg(test)]")?;
    writeln!(code, "mod generated_tests {{")?;
    if !bins.is_empty() || !examples.is_empty() {
        writeln!(code, "\tuse ::std::process::Command;")?;
        for bin in bins {
            writeln!(code, "\n\t#[test]")?;
            writeln!(code, "\tfn bin_{bin}_help() {{")?;
            writeln!(code, "\t\t")?;
            writeln!(code, "\t\tlet out = Command::new(\"cargo\").args([\"run\", \"--bin\", \"{bin}\", \"--all-features\", \"--\", \"--help\"])")?;
            writeln!(code, "\t\t\t.output().expect(\"failed to execute binary {bin}\");")?;
            writeln!(code, "\t\tassert_eq!(out.status.code(), Some(0), \"binary {bin} --help exit code was not 0\");")?;
            writeln!(code, "\t}}")?;
        }
        for example in examples {
            writeln!(code, "\n\t#[test]")?;
            writeln!(code, "\tfn example_{example}() {{")?;
            writeln!(code, "\t\tlet out = Command::new(\"cargo\").args([\"run\", \"--example\", \"{example}\", \"--all-features\"])")?;
            writeln!(code, "\t\t\t.output().expect(\"failed to execute example {example}\");")?;
            writeln!(code, "\t\tassert_eq!(out.status.code(), Some(0), \"example {example} exit code was not 0\");")?;
            writeln!(code, "\t}}")?;
        }
    }
    writeln!(code, "}}")?;
    let mut out_file = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_file.push("generated_tests.rs");
    fs::write(out_file, code).expect("failed to write");
    Ok(())
}

fn generate_base_dict_code(base_dict_entries: &[&str]) -> String {
    let mut init_buffer = String::with_capacity(base_dict_entries.len() * 16);
    let mut cost_buffer = String::new();
    writeln!(init_buffer, "pub const DICT: [DictEntry; {}] = [", base_dict_entries.len()).unwrap();
    for (pos, entry) in base_dict_entries.iter().enumerate() {
        let cost = encode_snippet_len_estimate(pos.try_into().unwrap());
        let creator = match *entry {
            "$magic-backspace$" => {
                writeln!(cost_buffer, "\t\t\tDictEntry::Backspace => {cost},").unwrap();
                "DictEntry::Backspace".to_owned()
            },
            "$magic-backspace-front$" => {
                writeln!(cost_buffer, "\t\t\tDictEntry::BackspaceFront => {cost},").unwrap();
                "DictEntry::BackspaceFront".to_owned()
            },
            "$magic-newline$" => format!("s(\"\\n\",{cost})"),
            "$magic-capitalize-first$" => {
                writeln!(cost_buffer, "\t\t\tDictEntry::CapitalizeFirst => {cost},").unwrap();
                "DictEntry::CapitalizeFirst".to_owned()
            },
            "$magic-capitalize-all$" => {
                writeln!(cost_buffer, "\t\t\tDictEntry::CapitalizeAll => {cost},").unwrap();
                "DictEntry::CapitalizeAll".to_owned()
            },
            "$magic-reverse$" => {
                writeln!(cost_buffer, "\t\t\tDictEntry::Reverse => {cost},").unwrap();
                "DictEntry::Reverse".to_owned()
            },
            "$magic-unicode$" => {
                writeln!(cost_buffer, "\t\t\tDictEntry::UnicodeLookup => {cost},").unwrap();
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
        writeln!(init_buffer, "\t{creator},").unwrap();
    }
    write!(init_buffer, "];\n\n").unwrap();
    writeln!(init_buffer, "impl DictEntry {{\n\tpub fn cost(&self) -> Cost {{\n\t\tmatch self {{\n\t\t\tDictEntry::Snippet {{ cost, .. }} => *cost,\n{cost_buffer}\t\t}}\n\t}}\n}}\n").unwrap();
    init_buffer
}

fn write_dict_code(code: &str) {
    let mut out_file = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_file.push("dict_init.rs");
    fs::write(out_file, code).expect("failed to write");
}
