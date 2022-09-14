#![feature(generic_associated_types)]

use ::std::env;
use std::process::ExitCode;
use crate::cli::run_tilde;

mod ast;
mod parse;
mod cli;
mod exec;

fn main() -> ExitCode {
    match run_tilde(env::args().collect()) {
        Ok(()) => ExitCode::from(0),
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::from(1)
        }
    }
}
