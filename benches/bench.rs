use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mandelbrot::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mandelbrot from (0,0) precision 100", |b| b.iter(|| mandelbrot(black_box(num::zero()), black_box(100))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);