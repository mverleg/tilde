use ::std::env;
use ::std::process::ExitCode;

use crate::cli::run_tilde;

mod cli;
mod common;

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
