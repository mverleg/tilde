
use ::criterion::{black_box, criterion_group, criterion_main, Criterion};
use ::tilde::run_tilde;
use ::tilde::TildeArgs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let args = TildeArgs {};
    c.bench_function("analyze_tilde_code", |b| b.iter(|| run_tilde(black_box(args))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
