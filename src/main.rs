#![feature(fmt_internals)]

use ::std::env;
use ::std::process::ExitCode;

use crate::cli::run_tilde;

mod ast;
mod cli;
mod common;
mod exec;
#[cfg(feature = "gen")]
mod gen;
mod parse;

pub type TildeRes<T> = Result<T, String>;

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
