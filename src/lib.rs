#![feature(fmt_internals)]
#![allow(unused)] //TODO @mark: TEMPORARY! REMOVE THIS!

use ::std::env;
use ::std::io;
use ::std::io::{BufReader, BufWriter};
use ::std::path::Path;
use ::std::process::ExitCode;

pub use crate::common::log as tilde_log;
pub use crate::exec::Value;

mod ast;
mod common;
mod compile;
mod exec;
#[cfg(feature = "gen")]
mod gen;
#[cfg(feature = "gen")]
pub use self::gen::mddoc;
#[cfg(not(feature = "gen"))]
fn gen_md_docs() -> TildeRes<()> {
    Err("doc-gen can only be used if compiled with feature `gen`".to_owned())
}

pub type TildeRes<T> = Result<T, String>;
pub type NR = f64;

/// Run tilde with input lines produces by a reader, and results handled by a writer.
pub fn tilde_from<R: io::Read, W: io::Write>(
    code: &str,
    reader: io::BufReader<R>,
    writer: io::BufWriter<W>,
) -> TildeRes<()> {
    todo!() //TODO @mark:
}

/// Run tilde with strings as input and output, useful for testing.
pub fn tilde_strs(code: &str, input: &str) -> TildeRes<String> {
    let mut output = vec![];
    tilde_from(
        code,
        BufReader::new(io::Cursor::new(input)),
        BufWriter::new(io::Cursor::new(&mut output)),
    )?;
    String::from_utf8(output).map_err(|err| format!("output was not utf8, err: {}", err))
}

/// Run a Tilde routine, taking a single Value as input and producing a single value
/// as output if successful, or failing with an error message if unsuccessful.
pub fn tilde_eval(code: &str, input: Value) -> TildeRes<Value> {
    todo!() //TODO @mark:
}
