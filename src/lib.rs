#![feature(fmt_internals)]
#![allow(unused)] //TODO @mark: TEMPORARY! REMOVE THIS!

use ::std::env;
use ::std::process::ExitCode;

use crate::exec::Value;

mod ast;
mod common;
mod compile;
mod exec;
#[cfg(feature = "gen")]
mod gen;

pub type TildeRes<T> = Result<T, String>;
pub type NR = f64;

/// Run a Tilde routine, taking a single Value as input and producing a single value
/// as output if successful, or failing with an error message if unsuccessful.
pub fn tilde(code: &str, input: Value) -> TildeRes<Value> {
    todo!() //TODO @mark:
}

/// Run tilde with input lines produces by a reader, and results handled by a writer.
pub fn tilde_from(code: &str, reader: (), writer: ()) -> TildeRes<()> {
    todo!() //TODO @mark:
}

//TODO @mark: does this allow
/// Run Tilde. The input is text, that will be split into lines, and the output is either
/// an error, or everything that would be printed to stdout. Useful for testing.
pub fn tilde_strs(code: &str, input: &str) -> TildeRes<String> {
    todo!() //TODO @mark:
}
