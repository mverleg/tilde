use ::std::fs::read_to_string;
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
use ::tilde::tilde_log;
use ::tilde::TildeRes;
use ::tilde::Value;

use crate::gen_help;

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

#[allow(unused)] //TODO @mark: TEMPORARY! REMOVE THIS!
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
