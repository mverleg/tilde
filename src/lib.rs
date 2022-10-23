#![feature(fmt_internals)]
#![allow(unused)] //TODO @mark: TEMPORARY! REMOVE THIS!

use ::std::env;
use ::std::io;
use ::std::path::Path;
use ::std::process::ExitCode;
use std::io::{BufReader, BufWriter};

pub use crate::exec::Value;

mod ast;
mod common;
mod compile;
mod exec;
#[cfg(feature = "gen")]
mod gen;

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
pub fn tilde_sts(code: &str, input: &str) -> TildeRes<String> {
    let mut output = String::new();
    tilde_from(code, BufReader::new(input), BufWriter::new(&mut output))?;
    Ok(output)
}

/// Run a Tilde routine, taking a single Value as input and producing a single value
/// as output if successful, or failing with an error message if unsuccessful.
pub fn tilde_eval(code: &str, input: Value) -> TildeRes<Value> {
    todo!() //TODO @mark:
}
