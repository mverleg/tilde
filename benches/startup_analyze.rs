use ::criterion::{black_box, Criterion, criterion_group, criterion_main};

use ::tilde::CliOperation;
use ::tilde::run_tilde;
use ::tilde::TildeArgs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let source = ",hello world";
    let args = TildeArgs { operation: CliOperation::Analyze(source.to_string()) };
    c.bench_function("analyze_tilde_code", |b| b.iter(|| run_tilde(black_box(&args))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
