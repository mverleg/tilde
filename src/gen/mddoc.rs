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
        if !opener.is_modifier() {
            gen_opener_doc(opener, &docs, ops)?;
        }
    }
    println!("created markdown docs");
    Ok(())
}

fn gen_index_doc(docs: &Vec<(Token, Vec<OpDoc>)>) -> TildeRes<()> {
    let mut docbuf = format!("\n# Tilde reference (v{})\n\n", env!("CARGO_PKG_VERSION"));
    let mut tlreadme = core::fmt::Formatter::new(&mut docbuf);
    write_openers(docs, &mut tlreadme);
    fs::write("doc/README.md", docbuf)
        .map_err(|err| format!("failed to write markdown index, err: {}", err))?;
    Ok(())
}

fn gen_opener_doc(opener: &Token, docs: &[(Token, Vec<OpDoc>)], ops: &[OpDoc]) -> TildeRes<()> {
    let mut docbuf = format!("\n# Tilde v{}: {}\n\n", env!("CARGO_PKG_VERSION"), opener.chr);
    let mut openfmt = core::fmt::Formatter::new(&mut docbuf);
    write_openers(&docs, &mut openfmt);
    fs::write(format!("doc/{}.md", opener.long), docbuf)
        .map_err(|err| format!("failed to write markdown opener doc, err: {}", err))?;
    Ok(())
}

fn write_openers(docs: &[(Token, Vec<OpDoc>)], tlreadme: &mut Formatter) {
    write!(tlreadme, "Openers: ").unwrap();
    let mut is_first = true;
    for (opener, _) in docs {
        if !opener.is_modifier() {
            if is_first {
                is_first = false;
            } else {
                write!(tlreadme, " | ").unwrap();
            }
            write!(tlreadme, "[{}](./{}.md)", opener.chr, &opener.long).unwrap();
        }
    }
    write!(tlreadme, "\n\n").unwrap();
}
