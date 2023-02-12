use ::std::fmt::Formatter;
use ::std::fs;

use crate::compile::Letter;
use crate::gen::doc::gen_grouped_docs;
use crate::gen::doc::OpDoc;
use crate::TildeRes;

pub fn tilde_gen_md_docs() -> TildeRes<()> {
    let docs = gen_grouped_docs();
    fs::create_dir_all("doc").map_err(|err| format!("failed to create doc directory, err: {}", err))?;
    gen_index_doc(&docs)?;
    for (opener, ops) in &docs {
        if opener.is_opener() {
            gen_opener_doc(opener, &docs, ops)?;
        }
    }
    println!("created markdown docs");
    Ok(())
}

fn gen_index_doc(docs: &[(Letter, Vec<OpDoc>)]) -> TildeRes<()> {
    let mut docbuf = format!("\n# Tilde reference (v{})\n\n", env!("CARGO_PKG_VERSION"));
    let mut tlreadme = core::fmt::Formatter::new(&mut docbuf);
    write_openers(docs, &mut tlreadme, u8::MAX);
    fs::write("doc/README.md", docbuf).map_err(|err| format!("failed to write markdown index, err: {}", err))?;
    Ok(())
}

fn gen_opener_doc(
    opener: &Letter,
    docs: &[(Letter, Vec<OpDoc>)],
    ops: &[OpDoc],
) -> TildeRes<()> {
    let mut docbuf = format!("\n# [Tilde](./README.md) v{}: opener {} ({})\n\n", env!("CARGO_PKG_VERSION"), opener.chr, opener.long);
    let mut openfmt = core::fmt::Formatter::new(&mut docbuf);
    write_openers(docs, &mut openfmt, opener.byte);
    writeln!(openfmt, "* Character: **{}** (#{:x}/{})", opener.chr, opener.byte, ALPHABET.len()).unwrap();
    writeln!(openfmt, "* Name: \"{}\"", &opener.long).unwrap();
    writeln!(
        openfmt,
        "* Type: {}",
        if opener.is_fixed() { "always 1 argument, and optional modifiers" } else { "no fixed argument, but allows optional modifiers" }
    )
    .unwrap();
    writeln!(openfmt, "\n### Operations:\n").unwrap();
    for op in ops {
        writeln!(openfmt, "* [{}](./{}.md)", op.chars(), op.op_name()).unwrap();
    }
    fs::write(format!("doc/{}.md", opener.long), docbuf).map_err(|err| format!("failed to write markdown opener doc, err: {}", err))?;
    Ok(())
}

fn write_openers(
    docs: &[(Letter, Vec<OpDoc>)],
    tlreadme: &mut Formatter,
    highlight: u8,
) {
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
