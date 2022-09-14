use ::std::fs::read_to_string;
use ::std::io::{BufRead, stdin};

use crate::exec::{execute, Value};

pub fn run_tilde(args: Vec<String>) -> Result<Value, String> {
    if let Some(source) = parse_args(args)? {
        let prog = parse(source);
        let inp = stdin().lock().lines().collect();
        execute(prog, inp)
    } else {
        Ok(gen_help().into())
    }
}

fn parse_args(args: Vec<String>) -> Result<Option<String>, String> {
    match args.get(1).map(|s| s.as_str()) {
        Some("-h") | Some("--help") => Ok(None),
        Some("-f") | Some("--file") => {
            let pth = args.get(2)
                .ok_or_else(|| format!("argument -f/--file expects a path to a source file"))?;
            Ok(Some(read_to_string(pth)
                .map_err(|err| format!("failed to read source file, err {err}"))?))
        },
        Some("-s") | Some("--source") => {
            Ok(Some(args.get(2)
                .ok_or_else(|| format!("argument -s/--source expects a single argument containing source code"))?
                .to_owned()))
        },
        Some(arg) => Err(format!("unknown argument '{arg}'; try --help for options")),
        None => Err(format!("expected at least one argument; try --help for options")),
    }
}

fn gen_help() -> String {
    "help here".to_owned()
}
