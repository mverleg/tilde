
use ::criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let args = TildeArgs {};
    c.bench_function("fib 20", |b| b.iter(|| run_tilde(black_box(args))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
