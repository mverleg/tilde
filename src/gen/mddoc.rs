use ::std::fs;

use crate::gen::doc::gen_grouped_docs;
use crate::TildeRes;

pub fn gen_md_docs() -> TildeRes<()> {
    let docs = gen_grouped_docs();
    let mut docbuf = format!("# Tilde reference (v{})\n\n", env!("CARGO_PKG_VERSION"));
    let mut tlreadme = core::fmt::Formatter::new(&mut docbuf);
    write!(tlreadme, "All 256 openers: ").unwrap();
    for (opener, groups) in docs {
        write!(tlreadme, "{} | ", opener.chr).unwrap();
    }
    fs::create_dir_all("doc")
        .map_err(|err| format!("failed to create doc directory, err: {}", err))?;
    fs::write("doc/README.md", docbuf)
        .map_err(|err| format!("failed to write doc readme directory, err: {}", err))?;
    Ok(())
}
