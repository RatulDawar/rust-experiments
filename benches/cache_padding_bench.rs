use cache_padding_demo::{benchmark_padded, benchmark_unpadded};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_unpadded(c: &mut Criterion) {
    c.bench_function("unpadded_counters", |b| {
        b.iter(|| {
            black_box(benchmark_unpadded());
        })
    });
}

fn bench_padded(c: &mut Criterion) {
    c.bench_function("padded_counters", |b| {
        b.iter(|| {
            black_box(benchmark_padded());
        })
    });
}

criterion_group!(benches, bench_unpadded, bench_padded);
criterion_main!(benches);
