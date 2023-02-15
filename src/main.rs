#![feature(fmt_internals)]

use ::std::env;
use ::std::fs::read_to_string;
use ::std::mem::size_of;
use ::std::process::ExitCode;
use ::std::str::from_utf8;
use ::base64::Engine;
use ::base64::engine::general_purpose::URL_SAFE_NO_PAD;

use ::tilde::CliOperation;
use ::tilde::run_tilde;
use ::tilde::tilde_log;
use ::tilde::TildeArgs;

fn main() -> ExitCode {
    assert!(size_of::<usize>() >= size_of::<u32>(), "due to indexing tricks, platforms with narrow pointers are not supported at this time");
    let operation: CliOperation = match parse_operation(env::args().collect()) {
        ArgParseRes::Lib(op) => op,
        ArgParseRes::GenHelp => {
            println!("{}", gen_help());
            return ExitCode::from(0)
        },
        ArgParseRes::Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(2)
        }
    };
    let args = TildeArgs { operation };
    let res = run_tilde(&args);
    match res {
        Ok(value) => {
            println!("{value}");
            ExitCode::from(0)
        },
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        },
    }
}

fn parse_operation(mut args: Vec<String>) -> ArgParseRes {
    use ArgParseRes::*;
    args.reverse();
    args.pop();
    let arg1 = args.pop();
    let cli_op = match arg1.as_deref() {
        Some("-h") | Some("--help") => GenHelp,
        Some("-f") | Some("--file") => {
            let Some(pth) = args.pop() else {
                return Err("argument -f/--file expects a path to a source file".to_string())
            };
            tilde_log!("reading source from file {pth}");
            let src = match read_to_string(pth) {
                Ok(src) => src,
                Result::Err(err) => return Err(format!("failed to read source file, err {err}"))
            };
            Lib(CliOperation::Run(src))
        },
        Some("-s") | Some("--source") => {
            let Some(src) = args.pop() else {
                return Err("argument -s/--source expects a single argument containing source code".to_string())
            };
            tilde_log!("getting source from command line (length in utf8 bytes: {})", src.len());
            Lib(CliOperation::Run(src))
        },
        Some("--base64source") => {
            let Some(b64src) = args.pop() else {
                return Err("argument --base64source expects a single argument containing base64-encoded source code".to_string())
            };
            tilde_log!("getting base64 source from command line (length in bytes: {})", b64src.len());
            let src = match URL_SAFE_NO_PAD.decode(b64src) {
                Ok(src_bytes) => {
                    let Ok(src) = from_utf8(&src_bytes) else {
                        tilde_log!("base64 decoded bytes: {:?}", &src_bytes);
                        return Err("source is not valid utf8 after base64-decoding; should contain valid, golfed tilde input, which is ascii".to_string())
                    };
                    src.to_owned()
                },
                Result::Err(_) => return Err("base64 encoding not valid, alphabet should be A-Za-z0-9-_ without padding".to_string()),
            };
            Lib(CliOperation::Run(src))
        },
        Some("-F") | Some("--analyze-file") => {
            let Some(pth) = args.pop() else {
                return Err("argument -F/--analyze-file expects a path to a source file".to_string())
            };
            tilde_log!("reading source from file {pth} for analysis");
            let src = match read_to_string(pth) {
                Ok(src) => src,
                Result::Err(err) => return Err(format!("failed to read source file, err {err}"))
            };
            Lib(CliOperation::Analyze(src))
        },
        Some("-S") | Some("--analyze-source") => {
            let Some(src) = args.pop() else {
                return Err("argument -S/--analyze-source expects a single argument containing source code".to_string())
            };
            tilde_log!("getting source from command line (length in utf8 bytes: {}) for analysis", src.len());
            Lib(CliOperation::Analyze(src))
        },
        Some("doc-gen") => Lib(CliOperation::DocGen),
        Some(arg) => {
            let hint = if arg.contains('=') { "hint: --arg=value syntax is not supported, use '--arg value'\n" } else { "" };
            return Err(format!("unknown argument '{arg}'\n{hint}try --help for options"))
        },
        None => Err("expected at least one argument; try --help for options".to_string()),
    };
    if !args.is_empty() {
        return Err(format!("cannot handle these extra arguments: {}\ntry --help for options", args.join(" ")));
    }
    cli_op
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
        "    --base64source B     Run base64-encoded source".to_owned(),
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

#[derive(Debug)]
enum ArgParseRes {
    Lib(CliOperation),
    Err(String),
    GenHelp,
}
