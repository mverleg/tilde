use ::std::fs::read_to_string;
use ::std::io::{BufRead, stdin};

use crate::exec::{execute, Value};
use crate::parse::parse;

pub fn run_tilde(args: Vec<String>) -> Result<Value, String> {
    if let Some(source) = parse_args(args)? {
        let prog = parse(&source)?;
        let inp = stdin().lock().lines()
            .map(|l| l.expect("cannot read line from stdin, not utf8?"))
            .collect();
        execute(prog, inp)
    } else {
        Ok(gen_help().into())
    }
}

fn parse_args(mut args: Vec<String>) -> Result<Option<String>, String> {
    args.reverse();
    args.pop();
    let arg1 = args.pop();
    let source = match arg1.as_deref() {
        Some("-h") | Some("--help") => Ok(None),
        Some("-f") | Some("--file") => {
            let pth = args.pop()
                .ok_or_else(|| format!("argument -f/--file expects a path to a source file"))?;
            Ok(Some(read_to_string(pth)
                .map_err(|err| format!("failed to read source file, err {err}"))?))
        },
        Some("-s") | Some("--source") => {
            Ok(Some(args.pop()
                .ok_or_else(|| format!("argument -s/--source expects a single argument containing source code"))?
                .to_owned()))
        },
        Some(arg) => {
            let hint = if arg.contains("=") {
                "hint: --arg=value syntax is not supported, use '--arg value'\n"
            } else {
                ""
            };
            Err(format!("unknown argument '{arg}'\n{hint}try --help for options"))
        },
        None => Err(format!("expected at least one argument; try --help for options")),
    }?;
    if !args.is_empty() {
        return Err(format!("cannot handle these arguments: {}\ntry --help for options", args.join(" ")))
    }
    Ok(source)
}

fn gen_help() -> String {
    vec![
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
        "    -h, --help        Show this help text".to_owned(),
        "    -s, --source S    Run source S, which should be golfed source with unicode encoding".to_owned(),
        "    -f, --file P      Run source contained in file at path P, which should be golfed".to_owned(),
        "                      source with unicode encoding".to_owned(),
    ].join("\n")
}
