use ::std::env;

pub enum ShowHelp { Show, NoShow }

pub fn run_tilde(args: Vec<String>) -> (ShowHelp, Result<(), String>) {
    parse_args(args)
}

fn parse_args(args: Vec<String>) -> (ShowHelp, Result<(), String>) {
    match args.get(1).map(|s| s.as_str()) {
        Some("-h") | Some("--help") => (ShowHelp::Show, Ok(())),
        Some("-f") | Some("--file") => (ShowHelp::NoShow, Ok(())),
        Some("-s") | Some("--source") => (ShowHelp::NoShow, Ok(())),
        Some(arg) => Err(format!("unknown argument '{arg}'; try --help for options")),
        None => Err(format!("expected at least one argument; try --help for options")),
    }
}
