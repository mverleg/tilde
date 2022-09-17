use ::std::fs::read_to_string;
use ::std::io::{stdin, BufRead};
use ::std::sync::atomic::{AtomicBool, Ordering};
use ::std::sync::Arc;
use ::std::thread;
use ::std::thread::sleep;
use ::std::time::Duration;

use crate::common::log;
use crate::exec::{execute, Value};
#[cfg(feature = "gen")]
use crate::gen::mddoc::gen_md_docs;
use crate::parse::parse;
use crate::TildeRes;

pub fn run_tilde(args: Vec<String>) -> TildeRes<Value> {
    if let Some(source) = parse_args(args)? {
        let inp = gather_input();
        let prog = parse(&source)?;
        execute(prog, inp)
    } else {
        Ok(gen_help().into())
    }
}

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

#[cfg(not(feature = "gen"))]
fn gen_md_docs() -> TildeRes<()> {
    Err("doc-gen can only be used if compiled with feature `gen`".to_owned())
}

fn parse_args(mut args: Vec<String>) -> TildeRes<Option<String>> {
    args.reverse();
    args.pop();
    let arg1 = args.pop();
    let source = match arg1.as_deref() {
        Some("-h") | Some("--help") => Ok(None),
        Some("-f") | Some("--file") => {
            let pth = args
                .pop()
                .ok_or_else(|| "argument -f/--file expects a path to a source file".to_string())?;
            log!("reading source from file {}", pth);
            Ok(Some(read_to_string(pth).map_err(|err| {
                format!("failed to read source file, err {err}")
            })?))
        }
        Some("-s") | Some("--source") => {
            let src = args.pop().ok_or_else(|| {
                "argument -s/--source expects a single argument containing source code".to_string()
            })?;
            log!(
                "getting source from command line (length in utf8 bytes: {})",
                src.len()
            );
            Ok(Some(src))
        }
        Some("doc-gen") => gen_md_docs(),
        Some(arg) => {
            let hint = if arg.contains('=') {
                "hint: --arg=value syntax is not supported, use '--arg value'\n"
            } else {
                ""
            };
            Err(format!(
                "unknown argument '{arg}'\n{hint}try --help for options"
            ))
        }
        None => Err("expected at least one argument; try --help for options".to_string()),
    }?;
    if !args.is_empty() {
        return Err(format!(
            "cannot handle these arguments: {}\ntry --help for options",
            args.join(" ")
        ));
    }
    Ok(source)
}

fn gen_help() -> String {
    let mut help = vec![
        format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        format!("{}", env!("CARGO_PKG_DESCRIPTION")),
        format!(
            "more info: {} ; by: {}",
            env!("CARGO_PKG_HOMEPAGE"),
            env!("CARGO_PKG_AUTHORS")
        ),
        "".to_owned(),
        "USAGE:".to_owned(),
        format!("    {} -s 'source here'", env!("CARGO_PKG_NAME")),
        format!("    {} --file ./source.~", env!("CARGO_PKG_NAME")),
        format!(
            "    cat input.txt | {} -s 'source here'",
            env!("CARGO_PKG_NAME")
        ),
        //TODO @mverleg: put better example source ^
        "".to_owned(),
        "OPTIONS:".to_owned(),
        "    -h, --help        Show this help text".to_owned(),
        "    -s, --source S    Run source S, which should be golfed source with unicode encoding"
            .to_owned(),
        "    -f, --file P      Run source contained in file at path P, which should be golfed"
            .to_owned(),
        "                      source with unicode encoding".to_owned(),
    ];
    if cfg!(feature = "gen") {
        help.push(
            "    doc-gen           Generate documentation (if built with `gen` feature)".to_owned(),
        );
    }
    help.join("\n")
}
