#![feature(fmt_internals)]
#![allow(unused)] //TODO @mark: TEMPORARY! REMOVE THIS!

use ::std::env;
use ::std::process::ExitCode;

use crate::cli::run_tilde;

mod ast;
mod cli;
mod common;
mod compile;
mod exec;
#[cfg(feature = "gen")]
mod gen;

pub type TildeRes<T> = Result<T, String>;
pub type NR = f64;

fn main() -> ExitCode {
    let res = run_tilde(env::args().collect());
    match res {
        Ok(value) => {
            println!("{}", value);
            ExitCode::from(0)
        }
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::from(1)
        }
    }
}
