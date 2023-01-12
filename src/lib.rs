#![feature(fmt_internals)]
#![feature(const_for)]
#![feature(once_cell)]
#![allow(unused)] //TODO @mark: TEMPORARY! REMOVE THIS!

use ::std::env;
use ::std::io;
use ::std::io::BufRead;
use ::std::io::BufReader;
use ::std::io::BufWriter;
use ::std::io::Read;
use ::std::io::stdin;
use ::std::path::Path;
use ::std::process::ExitCode;
use ::std::sync::Arc;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering;
use ::std::thread;
use ::std::thread::sleep;
use ::std::time::Duration;

pub use crate::common::log as tilde_log;
use crate::compile::{parse, Value};

#[cfg(feature = "gen")]
pub use self::gen::mddoc::tilde_gen_md_docs;

mod common;
mod compile;
mod exec;
#[cfg(feature = "gen")]
mod gen;
mod op;

#[cfg(not(feature = "gen"))]
pub fn tilde_gen_md_docs() -> TildeRes<()> {
    Err("doc-gen can only be used if compiled with feature `gen`".to_owned())
}

pub type TildeRes<T> = Result<T, String>;
pub type NR = f64;
pub type UINT = u64;

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
    DocGen,
}

//TODO @mverleg: move this code?
fn gather_input() -> Vec<String> {
    let is_ready = Arc::new(AtomicBool::new(false));
    let is_ready_clone = is_ready.clone();
    thread::spawn(move || {
        sleep(Duration::from_secs(5));
        if !is_ready_clone.load(Ordering::Acquire) {
            eprintln!("waiting for input on stdin; stdin needs to be closed before tilde can start")
        }
    });
    let inp = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("cannot read line from stdin, not utf8?"))
        .collect();
    is_ready.store(true, Ordering::Release);
    inp
}

/// Run tilde with input lines produces by a reader, and results handled by a writer.
pub fn tilde_from<R: io::Read, W: io::Write>(
    code: &str,
    reader: io::BufReader<R>,
    writer: io::BufWriter<W>,
) -> TildeRes<()> {
    let prog = parse(code)?;
    dbg!(&prog);  //TODO @mark: TEMPORARY! REMOVE THIS!
    todo!();
    Ok(())
}

/// Run tilde with strings as input and output, useful for testing.
pub fn tilde_strs(
    code: &str,
    input: &str,
) -> TildeRes<String> {
    let mut output = vec![];
    tilde_from(code, BufReader::new(io::Cursor::new(input)), BufWriter::new(io::Cursor::new(&mut output)))?;
    String::from_utf8(output).map_err(|err| format!("output was not utf8, err: {}", err))
}

/// Run a Tilde routine, taking a single Value as input and producing a single value
/// as output if successful, or failing with an error message if unsuccessful.
pub fn tilde_eval(
    code: &str,
    input: Value,
) -> TildeRes<Value> {
    todo!()
    //TODO @mark:
}

/// Analyze the Tilde source code and report stats as json.
pub fn tilde_analyze(code: &str) -> TildeRes<String> {
    let mut analysis = String::with_capacity(512);
    analysis.push_str("{\n");
    analysis.push_str("\"uses_preview_features\": false,\n");
    analysis.push_str("\"golf_code\": \"\",\n");
    analysis.push_str("\"base64_golf_code\": \"\",\n");
    analysis.push_str("\"long_command_code\": \"\",\n");
    analysis.push_str("\"length_valid\": 0,\n");
    analysis.push_str("\"length_preview_features\": 0\n");
    analysis.push_str("}\n");
    Ok(analysis)
}
