use ::std::fs;
use std::fmt::Formatter;

use crate::gen::doc::{gen_grouped_docs, OpDoc};
use crate::parse::{Token, TOKENSET};
use crate::TildeRes;

pub fn gen_md_docs() -> TildeRes<()> {
    let docs = gen_grouped_docs();
    let mut docbuf = format!("\n# Tilde reference (v{})\n\n", env!("CARGO_PKG_VERSION"));
    let mut tlreadme = core::fmt::Formatter::new(&mut docbuf);
    write_openers(docs, &mut tlreadme);
    fs::create_dir_all("doc")
        .map_err(|err| format!("failed to create doc directory, err: {}", err))?;
    fs::write("doc/README.md", docbuf)
        .map_err(|err| format!("failed to write doc readme directory, err: {}", err))?;
    println!("created markdown docs");
    Ok(())
}

fn write_openers(docs: Vec<(Token, Vec<OpDoc>)>, tlreadme: &mut Formatter) {
    write!(tlreadme, "Openers: ").unwrap();
    let mut is_first = true;
    for (opener, groups) in docs {
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
