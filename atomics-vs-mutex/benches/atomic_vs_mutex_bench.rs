use atomics_vs_mutex::{increment_with_atomic, increment_with_mutex};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench_atomic_vs_mutex(c: &mut Criterion) {
    let threads = [2_usize, 4, 8];
    let increments_per_thread_cases = [200_000_u64, 1_000_000_u64, 5_000_000_u64];

    for &increments_per_thread in &increments_per_thread_cases {
        let mut group =
            c.benchmark_group(format!("shared_counter/inc_per_thread_{increments_per_thread}"));

        for &thread_count in &threads {
            let total_ops = thread_count as u64 * increments_per_thread;
            group.throughput(Throughput::Elements(total_ops));

            group.bench_with_input(
                BenchmarkId::new("atomic_fetch_add_threads", thread_count),
                &thread_count,
                |b, &thread_count| {
                    b.iter(|| {
                        black_box(increment_with_atomic(thread_count, increments_per_thread));
                    });
                },
            );

            group.bench_with_input(
                BenchmarkId::new("mutex_lock_add_threads", thread_count),
                &thread_count,
                |b, &thread_count| {
                    b.iter(|| {
                        black_box(increment_with_mutex(thread_count, increments_per_thread));
                    });
                },
            );
        }

        group.finish();
    }
}

criterion_group!(benches, bench_atomic_vs_mutex);
criterion_main!(benches);
