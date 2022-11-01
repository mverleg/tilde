use ::std::fs::read_to_string;
use ::std::io::stdin;
use ::std::io::BufRead;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering;
use ::std::sync::Arc;
use ::std::thread;
use ::std::thread::sleep;
use ::std::time::Duration;

use ::tilde::mddoc::tilde_gen_md_docs;
use ::tilde::tilde_analyze;
use ::tilde::tilde_log;
use ::tilde::TildeRes;
use ::tilde::Value;

pub fn run_tilde(args: Vec<String>) -> TildeRes<Value> {
    match parse_args(args)? {
        CliOperation::Run(source) => {
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

fn gather_input() -> Vec<String> {
    let is_ready = Arc::new(AtomicBool::new(false));
    let is_ready_clone = is_ready.clone();
    thread::spawn(move || {
        sleep(Duration::from_secs(5));
        if !is_ready_clone.load(Ordering::Acquire) {
            eprintln!("waiting for input on stdin; stdin needs to be closed before tilde can start")
        }
    });
    let inp = stdin().lock().lines().map(|l| l.expect("cannot read line from stdin, not utf8?")).collect();
    is_ready.store(true, Ordering::Release);
    inp
}

#[derive(Debug)]
pub enum CliOperation {
    Run(String),
    Analyze(String),
    ShowHelp,
    DocGen,
}

fn parse_args(mut args: Vec<String>) -> TildeRes<CliOperation> {
    args.reverse();
    args.pop();
    let mut do_analyze = false;
    let mut cli_op: Option<CliOperation> = None;
    for arg in args {
        match arg.as_ref() {
            "-h" | "--help" => {
                cli_op = Some(CliOperation::ShowHelp);
                break;
            },
            "-f" | "--file" => {
                let pth = arg.ok_or_else(|| "argument -f/--file expects a path to a source file".to_string())?;
                tilde_log!("reading source from file {}", pth);
                if cli_op.is_some() {
                    return Err(format!("option --file conflicts with another argument"));
                }
                cli_op = Some(CliOperation::Run(read_to_string(pth).map_err(|err| format!("failed to read source file, err {err}"))?))
            },
            "-s" | "--source" => {
                let src = args.pop().ok_or_else(|| "argument -s/--source expects a single argument containing source code".to_string())?;
                tilde_log!("getting source from command line (length in utf8 bytes: {})", src.len());
                if cli_op.is_some() {
                    return Err(format!("option --source conflicts with another argument"));
                }
                cli_op = Some(CliOperation::Run(src))
            },
            "-a" | "--analyze" => do_analyze = true,
            "doc-gen" => {
                if cli_op.is_some() {
                    return Err(format!("subcommand `doc-gen` conflicts with another argument"));
                }
                cli_op = Some(CliOperation::DocGen)
            },
            arg => {
                let hint = if arg.contains('=') { "hint: --arg=value syntax is not supported, use '--arg value'\n" } else { "" };
                return Err(format!("unknown argument '{arg}'\n{hint}try --help for options"));
            },
        }
    }
    if do_analyze {
        if let Some(CliOperation::Run(code)) = cli_op {
            tilde_log!("got --analyze option, switching run task to analyze task");
            cli_op = Some(CliOperation::Analyze(code));
        } else {
            return Err(format!("option --analyze must be combined with --file or --source"));
        }
    }
    if !args.is_empty() {
        return Err(format!("cannot handle these arguments: {}\ntry --help for options", args.join(" ")));
    }
    cli_op.ok_or_else(|| "expected at least one of --help, --file, --source or doc-gen; try --help for options".to_string())
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
        "    -h, --help        Show this help text".to_owned(),
        "    -s, --source S    Run source string S (utf8)".to_owned(),
        "    -f, --file P      Run source contained in file at path P (utf8)".to_owned(),
        "    -a, --analyze     Show information about the program instead of running it".to_owned(),
    ];
    if cfg!(feature = "gen") {
        help.push("    doc-gen           Generate documentation (if built with `gen` feature)".to_owned());
    }
    help.join("\n")
}
