#![feature(fmt_internals)]
#![feature(const_for)]
#![feature(once_cell)]
#![feature(hasher_prefixfree_extras)]
#![feature(iter_intersperse)]
#![feature(variant_count)]

#![allow(unused_variables)]  //TODO @mark: TEMPORARY! REMOVE THIS!
#![allow(unreachable_code)]  //TODO @mark: TEMPORARY! REMOVE THIS!
#![allow(dead_code)]  //TODO @mark: TEMPORARY! REMOVE THIS!
#![allow(clippy::new_ret_no_self)]
#![allow(clippy::len_without_is_empty)]

use ::std::io;
use ::std::io::BufRead;
use ::std::io::Read;
use ::std::io::stdin;
use ::std::io::Write;
use ::std::sync::Arc;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering;
use ::std::thread;
use ::std::thread::sleep;
use ::std::time::Duration;

use crate::common::escape_for_string;
use crate::common::is_safe_for_string;
pub use crate::common::log as tilde_log;
use crate::compile::parse;
use crate::dict::ALLOW_COMPRESSION;
use crate::exec::execute;

pub use self::data::Array;
pub use self::data::Nr;
pub use self::data::Text;
pub use self::data::Value;
pub use self::data::Values;

// #[cfg(feature = "gen")]
// pub use self::gen::mddoc::tilde_gen_md_docs;
//TODO @mark: ^ fix and enable `gen`

mod common;
mod compile;
mod exec;
mod data;
//#[cfg(feature = "gen")]
//mod gen;
//TODO @mark: ^ fix and enable `gen`
mod op;
mod dict;

#[cfg(not(feature = "gen"))]
pub fn tilde_gen_md_docs() -> TildeRes<()> {
    Err("doc-gen can only be used if compiled with feature `gen`".to_owned())
}

pub type TildeRes<T> = Result<T, String>;
pub type UINT = u64;

pub fn run_tilde(args: &TildeArgs) -> TildeRes<Value> {
    match &args.operation {
        CliOperation::Run(source, mode) => {
            ALLOW_COMPRESSION.store(false, Ordering::Release);
            //tilde_from();
            let inp = gather_input();
            let res = tilde_strs_mode(source, &inp, *mode)?;
            Ok(Value::Txt(Text::of(res)))
            //TODO @mark: change to tilde_from ^
        },
        CliOperation::Analyze(source) =>
            Ok(tilde_analyze(source)?.into()),
        CliOperation::DocGen => {
            //tilde_gen_md_docs()?;
            todo!();  //TODO @mark: ^ fix and enable `gen`
            Ok(Value::default())
        },
    }
}

#[derive(Debug)]
pub struct TildeArgs {
    pub operation: CliOperation,
}

#[derive(Debug)]
pub enum CliOperation {
    Run(String, RunMode),
    Analyze(String),
    DocGen,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RunMode {
    Any,
    GolfOnly,
}

impl RunMode {
    pub fn golf_only(&self) -> bool {
        *self == RunMode::GolfOnly
    }
}

//TODO @mverleg: move this code?
fn gather_input() -> String {
    let is_ready = Arc::new(AtomicBool::new(false));
    let is_ready_clone = is_ready.clone();
    thread::spawn(move || {
        sleep(Duration::from_secs(5));
        if !is_ready_clone.load(Ordering::Acquire) {
            eprintln!("waiting for input on stdin; stdin needs to be closed before tilde can start")
        }
    });
    let mut inp = String::new();
    stdin().read_to_string(&mut inp)
        .expect("failed to read stdin");  //TODO @mark:
    is_ready.store(true, Ordering::Release);
    inp
}

/// Run tilde with input lines produces by a reader, and results handled by a writer.
pub fn tilde_from<R: io::Read, W: io::Write>(
    code: &str,
    reader: io::BufReader<R>,
    writer: io::BufWriter<W>,
    mode: RunMode,
) -> TildeRes<()> {
    let prog = parse(code, mode)?;
    let val = execute(prog, build_input(reader))?;
    let mut writer = writer;
    tilde_log!("tilde result: {}", val);
    write!(writer, "{}", val).unwrap();
    Ok(())
}

fn build_input<R: io::Read>(mut reader: io::BufReader<R>) -> Value {
    let mut lines = Vec::new();
    let mut line = String::new();
    while let Ok(siz) = reader.read_line(&mut line) {
        if siz == 0 {
            break
        }
        lines.push(Value::Txt(Text::of(&line)));
    }
    Value::Arr(Array::of(lines))
}

/// Run tilde with strings as input and output, useful for testing.
pub fn tilde_strs(
    code: &str,
    input: &str,
) -> TildeRes<String> {
    tilde_strs_mode(code, input, RunMode::Any)
}

pub fn tilde_strs_mode(
    code: &str,
    input: &str,
    mode: RunMode,
) -> TildeRes<String> {
    let mut output = vec![];
    tilde_from(
        code,
        io::BufReader::new(io::Cursor::new(input)),
        io::BufWriter::new(io::Cursor::new(&mut output)),
        mode)?;
    String::from_utf8(output).map_err(|err| format!("output was not utf8, err: {err}"))
}

// /// Run a Tilde routine, taking a single Value as input and producing a single value
// /// as output if successful, or failing with an error message if unsuccessful.
// pub fn tilde_eval(
//     code: &str,
//     input: Value,
// ) -> TildeRes<Value> {
//     todo!()
//     //TODO @mark:
// }
//TODO @mark: TEMPORARY! REMOVE THIS!

/// Analyze the Tilde source code and report stats as json.
pub fn tilde_analyze(source: &str) -> TildeRes<String> {
    //use crate::dict::compress_with_dict;
    let prog = parse(source, RunMode::Any)?;
    let unsafe_long_code = prog.long_code();
    let golf_code = prog.golf_code()?;
    let base64_code = prog.golf_code_b64()?;
    debug_assert!(is_safe_for_string(&base64_code), "base64 code should never contain double quotes or trailing backslashes");
    let safe_long_code = escape_for_string(unsafe_long_code);
    let safe_golf_code = escape_for_string(golf_code);
    let mut analysis = String::with_capacity(512);
    analysis.push_str("{\n");
    analysis.push_str("\"uses_preview_features\": false,\n");
    analysis.push_str("\"golf_code\": \"");
    analysis.push_str(&safe_golf_code);
    analysis.push_str("\",\n");
    analysis.push_str("\"base64_golf_code\": \"");
    analysis.push_str(&base64_code);
    analysis.push_str("\",\n");
    analysis.push_str("\"long_command_code\": \"");
    analysis.push_str(&safe_long_code);
    analysis.push_str("\",\n");
    analysis.push_str("\"length_valid_in_bytes\": ");
    analysis.push_str(&format!("{}", -1));
    analysis.push_str(",\n");
    analysis.push_str("\"length_preview_features_in_bytes\": ");
    analysis.push_str(&format!("{}", prog.golf_len()?));
    analysis.push_str("\n}\n");
    //TODO @mverleg: explanation
    //if 1 == 1 { return Ok(compress_with_dict(code).len().to_string()); }
    Ok(analysis)
    //TODO @mverleg: implement
}
