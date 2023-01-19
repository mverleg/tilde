use ::std::thread;

use ::criterion::{black_box, Criterion, criterion_group, criterion_main};

use ::tilde::CliOperation;
use ::tilde::run_tilde;
use ::tilde::TildeArgs;

//TODO @mverleg: remove? or start using

pub fn criterion_benchmark(c: &mut Criterion) {
    let source = ",hello world";
    c.bench_function("analyze_tilde_code", |b| b.iter(|| thread::spawn(||
        run_tilde(black_box(&TildeArgs { operation: CliOperation::Analyze(source.to_string()) }))
    ).join().unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
