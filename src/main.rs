#![feature(once_cell)]

use ::std::env;
use ::std::process::ExitCode;

use crate::cli::run_tilde;

mod common;
mod ast;
mod parse;
mod cli;
mod exec;

fn main() -> ExitCode {
    match run_tilde(env::args().collect()) {
        Ok(value) => {
            println!("{}", value);
            ExitCode::from(0)
        },
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::from(1)
        }
    }
}
