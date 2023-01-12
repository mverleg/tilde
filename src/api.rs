use ::std::io::BufRead;
use ::std::io::stdin;
use ::std::sync::Arc;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering;
use ::std::thread;
use ::std::thread::sleep;
use ::std::time::Duration;

use ::tilde::tilde_analyze;
use ::tilde::tilde_gen_md_docs;
use ::tilde::TildeRes;
use ::tilde::Value;

use crate::gen_help;

#[cfg(not(feature = "gen"))]
pub fn tilde_gen_md_docs() -> TildeRes<()> {
    Err("doc-gen can only be used if compiled with feature `gen`".to_owned())
}

pub type TildeRes<T> = Result<T, String>;
pub type NR = f64;
pub type UINT = u64;

#[cfg(feature = "gen")]
pub use self::gen::mddoc::tilde_gen_md_docs;

pub fn run_tilde(args: &TildeArgs) -> TildeRes<Value> {
    match &args.operation {
        CliOperation::Run(_source) => {
            todo!() //TODO @mark: TEMPORARY! REMOVE THIS!
            //tilde_from();
            // let inp = gather_input();
            // let prog = parse(&source)?;
            // execute(prog, inp)
        },
        CliOperation::Analyze(source) => Ok(tilde_analyze(&source)?.into()),
        CliOperation::ShowHelp => Ok(gen_help().into()),
        CliOperation::DocGen => {
            tilde_gen_md_docs()?;
            Ok(Value::None)
        },
    }
}

#[derive(Debug)]
pub struct TildeArgs {
    pub operation: CliOperation,
}

#[derive(Debug)]
pub enum CliOperation {
    Run(String),
    Analyze(String),
    ShowHelp,
    DocGen,
}
