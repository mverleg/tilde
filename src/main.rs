#![feature(fmt_internals)]

use ::std::env;
use ::std::fs::read_to_string;
use ::std::process::ExitCode;

use ::tilde::tilde_log;
use ::tilde::TildeRes;

use crate::cli::{CliOperation, run_tilde, TildeArgs};

mod cli;

fn main() -> ExitCode {
    let operation = match parse_operation(env::args().collect()) {
        Ok(op) => op,
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::from(2)
        }
    };
    let args = TildeArgs { operation };
    let res = run_tilde(&args);
    match res {
        Ok(value) => {
            println!("{}", value);
            ExitCode::from(0)
        },
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::from(1)
        },
    }
}

fn parse_operation(mut args: Vec<String>) -> TildeRes<CliOperation> {
    args.reverse();
    args.pop();
    let arg1 = args.pop();
    let cli_op: CliOperation = match arg1.as_deref() {
        Some("-h") | Some("--help") => Ok(CliOperation::ShowHelp),
        Some("-f") | Some("--file") => {
            let pth = args
                .pop()
                .ok_or_else(|| "argument -f/--file expects a path to a source file".to_string())?;
            tilde_log!("reading source from file {}", pth);
            Ok(CliOperation::Run(read_to_string(pth).map_err(|err| format!("failed to read source file, err {err}"))?))
        },
        Some("-s") | Some("--source") => {
            let src = args
                .pop()
                .ok_or_else(|| "argument -s/--source expects a single argument containing source code".to_string())?;
            tilde_log!("getting source from command line (length in utf8 bytes: {})", src.len());
            Ok(CliOperation::Run(src))
        },
        Some("-F") | Some("--analyze-file") => {
            let pth = args
                .pop()
                .ok_or_else(|| "argument -F/--analyze-file expects a path to a source file".to_string())?;
            tilde_log!("reading source from file {} for analysis", pth);
            Ok(CliOperation::Analyze(read_to_string(pth).map_err(|err| format!("failed to read source file, err {err}"))?))
        },
        Some("-S") | Some("--analyze-source") => {
            let src = args
                .pop()
                .ok_or_else(|| "argument -S/--analyze-source expects a single argument containing source code".to_string())?;
            tilde_log!("getting source from command line (length in utf8 bytes: {}) for analysis", src.len());
            Ok(CliOperation::Analyze(src))
        },
        Some("doc-gen") => Ok(CliOperation::DocGen),
        Some(arg) => {
            let hint = if arg.contains('=') { "hint: --arg=value syntax is not supported, use '--arg value'\n" } else { "" };
            Err(format!("unknown argument '{arg}'\n{hint}try --help for options"))
        },
        None => Err("expected at least one argument; try --help for options".to_string()),
    }?;
    if !args.is_empty() {
        return Err(format!("cannot handle these arguments: {}\ntry --help for options", args.join(" ")));
    }
    Ok(cli_op)
}

fn gen_help() -> String {
    let mut help = vec![
        format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        format!("{}", env!("CARGO_PKG_DESCRIPTION")),
        format!("more info: {} ; by: {}", env!("CARGO_PKG_HOMEPAGE"), env!("CARGO_PKG_AUTHORS")),
        "".to_owned(),
        "USAGE:".to_owned(),
        format!("    {} -s 'source here'", env!("CARGO_PKG_NAME")),
        format!("    {} --file ./source.~", env!("CARGO_PKG_NAME")),
        format!("    cat input.txt | {} -s 'source here'", env!("CARGO_PKG_NAME")),
        //TODO @mverleg: put better example source ^
        "".to_owned(),
        "OPTIONS:".to_owned(),
        "    -h, --help           Show this help text".to_owned(),
        "    -s, --source S       Run source string S (utf8)".to_owned(),
        "    -f, --file P         Run source contained in file at path P (utf8)".to_owned(),
        "    -S, --analyze-source Show information about the source string instead of running it".to_owned(),
        "    -F, --analyze-file   Show information about the source file instead of running it".to_owned(),
        // "    --stats           Show stats (json) about the program instead of running it"
        //     .to_owned(),
    ];
    if cfg!(feature = "gen") {
        help.push("    doc-gen              Generate documentation (if built with `gen` feature)".to_owned());
    }
    help.join("\n")
}
