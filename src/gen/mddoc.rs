use ::std::fs;
use std::fmt::Formatter;

use crate::gen::doc::{gen_grouped_docs, OpDoc};
use crate::parse::{Token, TOKENSET};
use crate::TildeRes;

pub fn gen_md_docs() -> TildeRes<()> {
    let docs = gen_grouped_docs();
    fs::create_dir_all("doc")
        .map_err(|err| format!("failed to create doc directory, err: {}", err))?;
    gen_index_doc(&docs)?;
    for (opener, ops) in &docs {
        if opener.is_opener() {
            gen_opener_doc(opener, &docs, ops)?;
        }
    }
    println!("created markdown docs");
    Ok(())
}

fn gen_index_doc(docs: &Vec<(Token, Vec<OpDoc>)>) -> TildeRes<()> {
    let mut docbuf = format!("\n# Tilde reference (v{})\n\n", env!("CARGO_PKG_VERSION"));
    let mut tlreadme = core::fmt::Formatter::new(&mut docbuf);
    write_openers(docs, &mut tlreadme, u8::MAX);
    fs::write("doc/README.md", docbuf)
        .map_err(|err| format!("failed to write markdown index, err: {}", err))?;
    Ok(())
}

fn gen_opener_doc(opener: &Token, docs: &[(Token, Vec<OpDoc>)], ops: &[OpDoc]) -> TildeRes<()> {
    let mut docbuf = format!("\n# [Tilde](./README.md) v{}: opener {} ({})\n\n", env!("CARGO_PKG_VERSION"), opener.chr, opener.long);
    let mut openfmt = core::fmt::Formatter::new(&mut docbuf);
    write_openers(&docs, &mut openfmt, opener.byte);
    write!(openfmt, "* Character: **{}** (#{:x}/{})\n", opener.chr, opener.byte, TOKENSET.len()).unwrap();
    write!(openfmt, "* Name: \"{}\"\n", &opener.long).unwrap();
    write!(openfmt, "* Type: {}\n", if opener.is_fixed() {
        "always 1 argument, and optional modifiers"
    } else {
        "no fixed argument, but allows optional modifiers"
    }).unwrap();
    write!(openfmt, "\n### Operations:\n\n").unwrap();
    for op in ops {
        write!(openfmt, "* [{}](./{}.md)\n", op.chars(), op.op_name()).unwrap();
    }
    fs::write(format!("doc/{}.md", opener.long), docbuf)
        .map_err(|err| format!("failed to write markdown opener doc, err: {}", err))?;
    Ok(())
}

fn write_openers(docs: &[(Token, Vec<OpDoc>)], tlreadme: &mut Formatter, highlight: u8) {
    write!(tlreadme, "Openers: ").unwrap();
    let mut is_first = true;
    for (opener, _) in docs {
        if opener.is_opener() {
            if is_first {
                is_first = false;
            } else {
                write!(tlreadme, " | ").unwrap();
            }
            if highlight == opener.byte {
                write!(tlreadme, "**{}**", opener.chr).unwrap();
            } else {
                write!(tlreadme, "[{}](./{}.md)", opener.chr, &opener.long).unwrap();
            }
        }
    }
    write!(tlreadme, "\n\n").unwrap();
}
