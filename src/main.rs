#![feature(generic_associated_types)]

use ::std::env;

use ::regex::Regex;

mod ast;
mod parse;
mod cli;
mod exec;

fn main() {
    reject_args();
    println!("Hello, world!");
    assert!(Regex::new("^.+$").unwrap().is_match("a"));
}

fn reject_args() {
    let mut args = env::args();
    args.next();
    let arg1 = args.next();
    if let Some(arg1) = arg1 {
        eprintln!("did not expect any arguments, got {}", arg1);
    }
}
